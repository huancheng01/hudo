// GPU 流体极光暗流背景 —— cappen-fluid-simulation 的裸 WebGL2 移植（去 three.js）
// 全屏 quad FBO ping-pong: splat → curl → vorticity → divergence → pressure(Jacobi) → gradientSubtract → advection
// 只做 WebGL2 + EXT_color_buffer_float, 创建失败返回 null 由调用方回退静态背景

const VERT = `
attribute vec2 position;
varying vec2 vUv;
void main() { vUv = position * .5 + .5; gl_Position = vec4(position, 0., 1.); }`

const FRAG = {
  splat: `precision highp float;
uniform sampler2D uTarget; uniform float aspectRatio, radius; uniform vec3 color; uniform vec2 point; varying vec2 vUv;
void main() { vec2 p = vUv - point; p.x *= aspectRatio; gl_FragColor = vec4(texture2D(uTarget, vUv).xyz + exp(-dot(p, p) / radius) * color, 1.); }`,

  advection: `precision highp float;
uniform sampler2D uVelocity, uSource; uniform vec2 texelSize; uniform float dt, dissipation; varying vec2 vUv;
void main() { gl_FragColor = vec4(dissipation * texture2D(uSource, vUv - dt * texture2D(uVelocity, vUv).xy * texelSize).rgb, 1.); }`,

  divergence: `precision highp float;
uniform sampler2D uVelocity; uniform vec2 texelSize; varying vec2 vUv;
vec2 vel(vec2 uv) { vec2 e = vec2(1.); if (uv.x < 0.) { uv.x = 0.; e.x = -1.; } if (uv.x > 1.) { uv.x = 1.; e.x = -1.; } if (uv.y < 0.) { uv.y = 0.; e.y = -1.; } if (uv.y > 1.) { uv.y = 1.; e.y = -1.; } return e * texture2D(uVelocity, uv).xy; }
void main() { vec2 L = vUv - vec2(texelSize.x, 0.), R = vUv + vec2(texelSize.x, 0.), T = vUv + vec2(0., texelSize.y), B = vUv - vec2(0., texelSize.y); gl_FragColor = vec4(.5 * (vel(R).x - vel(L).x + vel(T).y - vel(B).y), 0., 0., 1.); }`,

  curl: `precision highp float;
uniform sampler2D uVelocity; uniform vec2 texelSize; varying vec2 vUv;
void main() { vec2 L = vUv - vec2(texelSize.x, 0.), R = vUv + vec2(texelSize.x, 0.), T = vUv + vec2(0., texelSize.y), B = vUv - vec2(0., texelSize.y); gl_FragColor = vec4(texture2D(uVelocity, R).y - texture2D(uVelocity, L).y - texture2D(uVelocity, T).x + texture2D(uVelocity, B).x, 0., 0., 1.); }`,

  vorticity: `precision highp float;
uniform sampler2D uVelocity, uCurl; uniform vec2 texelSize; uniform float curlStrength, dt; varying vec2 vUv;
void main() { vec2 L = vUv - vec2(texelSize.x, 0.), R = vUv + vec2(texelSize.x, 0.), T = vUv + vec2(0., texelSize.y), B = vUv - vec2(0., texelSize.y); vec2 f = normalize(vec2(abs(texture2D(uCurl, T).x) - abs(texture2D(uCurl, B).x), abs(texture2D(uCurl, R).x) - abs(texture2D(uCurl, L).x)) + .0001) * curlStrength * texture2D(uCurl, vUv).x; gl_FragColor = vec4(texture2D(uVelocity, vUv).xy + f * dt, 0., 1.); }`,

  pressure: `precision highp float;
uniform sampler2D uPressure, uDivergence; uniform vec2 texelSize; varying vec2 vUv;
void main() { vec2 L = clamp(vUv - vec2(texelSize.x, 0.), 0., 1.), R = clamp(vUv + vec2(texelSize.x, 0.), 0., 1.), T = clamp(vUv + vec2(0., texelSize.y), 0., 1.), B = clamp(vUv - vec2(0., texelSize.y), 0., 1.); gl_FragColor = vec4((texture2D(uPressure, L).x + texture2D(uPressure, R).x + texture2D(uPressure, T).x + texture2D(uPressure, B).x - texture2D(uDivergence, vUv).x) * .25, 0., 0., 1.); }`,

  gradientSubtract: `precision highp float;
uniform sampler2D uPressure, uVelocity; uniform vec2 texelSize; varying vec2 vUv;
void main() { float pL = texture2D(uPressure, clamp(vUv - vec2(texelSize.x, 0.), 0., 1.)).x, pR = texture2D(uPressure, clamp(vUv + vec2(texelSize.x, 0.), 0., 1.)).x, pT = texture2D(uPressure, clamp(vUv + vec2(0., texelSize.y), 0., 1.)).x, pB = texture2D(uPressure, clamp(vUv - vec2(0., texelSize.y), 0., 1.)).x; gl_FragColor = vec4(texture2D(uVelocity, vUv).xy - vec2(pR - pL, pT - pB), 0., 1.); }`,

  clear: `precision highp float;
uniform sampler2D uTexture; uniform float value; varying vec2 vUv;
void main() { gl_FragColor = value * texture2D(uTexture, vUv); }`,

  // 显示层: 弃原版单色硬边墨迹, 改柔光染料场(tonemap 压亮度, 峰值 ≤~0.3)
  display: `precision highp float;
uniform sampler2D uTexture; varying vec2 vUv;
void main() {
  vec3 c = texture2D(uTexture, vUv).rgb;
  c = c / (1. + c);
  float a = clamp(max(c.r, max(c.g, c.b)) * 1.5, 0., .55);
  gl_FragColor = vec4(c * 1.15, a);
}`,
}

// 亮度压制靠低强度 splat 而非后期衰减: 三品牌色 * ~0.3
const BRAND = [
  [0.23 * 0.9, 0.51 * 0.9, 0.96 * 0.9],   // #3b82f6
  [0.55 * 0.85, 0.36 * 0.85, 0.96 * 0.85], // #8b5cf6
  [0.93 * 0.7, 0.28 * 0.7, 0.6 * 0.7],     // #ec4899
]

const CONF = {
  simRes: 128,
  dyeRes: 512,
  curl: 28,                    // 涡旋感; 低了拖尾是均匀香肠
  pressureIterations: 20,
  velocityDissipation: 0.985,
  dyeDissipation: 0.962,       // 染料快衰减→游丝极光; 慢了会积成持久色块(熔岩灯)
  pressureDecay: 0.8,
}

export function createFluid(canvas, { onLive, onFallback } = {}) {
  const gl = canvas.getContext('webgl2', { alpha: true, depth: false, stencil: false, antialias: false, powerPreference: 'low-power' })
  if (!gl) return null
  if (!gl.getExtension('EXT_color_buffer_float')) return null

  const dpr = Math.min(window.devicePixelRatio || 1, 1.5)

  function compile(type, src) {
    const sh = gl.createShader(type)
    gl.shaderSource(sh, src)
    gl.compileShader(sh)
    if (!gl.getShaderParameter(sh, gl.COMPILE_STATUS)) return null
    return sh
  }
  const vs = compile(gl.VERTEX_SHADER, VERT)
  if (!vs) return null

  function program(fragSrc) {
    const fs = compile(gl.FRAGMENT_SHADER, fragSrc)
    if (!fs) return null
    const p = gl.createProgram()
    gl.attachShader(p, vs)
    gl.attachShader(p, fs)
    gl.bindAttribLocation(p, 0, 'position')
    gl.linkProgram(p)
    if (!gl.getProgramParameter(p, gl.LINK_STATUS)) return null
    // 显式按名取 location, 不走 getActiveUniform 枚举(多变量单行声明下有枚举怪癖)
    const uniforms = {}
    const re = /uniform\s+\w+\s+([^;]+);/g
    let m
    while ((m = re.exec(fragSrc))) {
      for (const raw of m[1].split(',')) {
        const name = raw.trim()
        if (name) uniforms[name] = gl.getUniformLocation(p, name)
      }
    }
    return { p, uniforms }
  }

  const progs = {}
  for (const k in FRAG) {
    progs[k] = program(FRAG[k])
    if (!progs[k]) return null
  }

  const quad = gl.createBuffer()
  gl.bindBuffer(gl.ARRAY_BUFFER, quad)
  gl.bufferData(gl.ARRAY_BUFFER, new Float32Array([-1, -1, 1, -1, -1, 1, 1, 1]), gl.STATIC_DRAW)
  gl.enableVertexAttribArray(0)
  gl.vertexAttribPointer(0, 2, gl.FLOAT, false, 0, 0)

  function fbo(w, h) {
    const tex = gl.createTexture()
    gl.bindTexture(gl.TEXTURE_2D, tex)
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.LINEAR)
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.LINEAR)
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE)
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE)
    gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA16F, w, h, 0, gl.RGBA, gl.HALF_FLOAT, null)
    const fb = gl.createFramebuffer()
    gl.bindFramebuffer(gl.FRAMEBUFFER, fb)
    gl.framebufferTexture2D(gl.FRAMEBUFFER, gl.COLOR_ATTACHMENT0, gl.TEXTURE_2D, tex, 0)
    gl.clearColor(0, 0, 0, 0)
    gl.clear(gl.COLOR_BUFFER_BIT)
    return { tex, fb, w, h }
  }
  const double = (w, h) => ({ read: fbo(w, h), write: fbo(w, h), swap() { [this.read, this.write] = [this.write, this.read] } })

  let velocity, dye, divergence, curlFbo, pressure, simSize, dyeSize
  function allocTargets() {
    const aspect = canvas.width / Math.max(1, canvas.height)
    simSize = { w: CONF.simRes, h: Math.max(1, Math.round(CONF.simRes / aspect)) }
    dyeSize = { w: CONF.dyeRes, h: Math.max(1, Math.round(CONF.dyeRes / aspect)) }
    velocity = double(simSize.w, simSize.h)
    dye = double(dyeSize.w, dyeSize.h)
    divergence = fbo(simSize.w, simSize.h)
    curlFbo = fbo(simSize.w, simSize.h)
    pressure = double(simSize.w, simSize.h)
  }
  function freeTargets() {
    for (const t of [velocity?.read, velocity?.write, dye?.read, dye?.write, divergence, curlFbo, pressure?.read, pressure?.write]) {
      if (t) { gl.deleteTexture(t.tex); gl.deleteFramebuffer(t.fb) }
    }
  }

  function sizeCanvas() {
    canvas.width = Math.max(1, Math.floor(canvas.clientWidth * dpr))
    canvas.height = Math.max(1, Math.floor(canvas.clientHeight * dpr))
  }
  sizeCanvas()
  allocTargets()

  function pass(prog, target) {
    gl.useProgram(prog.p)
    gl.bindFramebuffer(gl.FRAMEBUFFER, target ? target.fb : null)
    gl.viewport(0, 0, target ? target.w : canvas.width, target ? target.h : canvas.height)
    gl.drawArrays(gl.TRIANGLE_STRIP, 0, 4)
  }
  function bindTex(prog, name, tex, unit) {
    gl.activeTexture(gl.TEXTURE0 + unit)
    gl.bindTexture(gl.TEXTURE_2D, tex)
    gl.uniform1i(prog.uniforms[name], unit)
  }

  // splat: point/velocity 均在 UV 空间(y 向上)
  function splat(x, y, vx, vy, color, radius) {
    const s = progs.splat
    gl.useProgram(s.p)
    gl.uniform1f(s.uniforms.aspectRatio, canvas.width / canvas.height)
    gl.uniform2f(s.uniforms.point, x, y)
    gl.uniform1f(s.uniforms.radius, radius)
    bindTex(s, 'uTarget', velocity.read.tex, 0)
    gl.uniform3f(s.uniforms.color, vx, vy, 0)
    pass(s, velocity.write)
    velocity.swap()
    bindTex(s, 'uTarget', dye.read.tex, 0)
    gl.uniform3f(s.uniforms.color, color[0], color[1], color[2])
    pass(s, dye.write)
    dye.swap()
  }

  function simulate(dt) {
    const tx = 1 / simSize.w, ty = 1 / simSize.h
    let p = progs.curl
    gl.useProgram(p.p)
    gl.uniform2f(p.uniforms.texelSize, tx, ty)
    bindTex(p, 'uVelocity', velocity.read.tex, 0)
    pass(p, curlFbo)

    p = progs.vorticity
    gl.useProgram(p.p)
    gl.uniform2f(p.uniforms.texelSize, tx, ty)
    gl.uniform1f(p.uniforms.curlStrength, CONF.curl)
    gl.uniform1f(p.uniforms.dt, dt)
    bindTex(p, 'uVelocity', velocity.read.tex, 0)
    bindTex(p, 'uCurl', curlFbo.tex, 1)
    pass(p, velocity.write)
    velocity.swap()

    p = progs.divergence
    gl.useProgram(p.p)
    gl.uniform2f(p.uniforms.texelSize, tx, ty)
    bindTex(p, 'uVelocity', velocity.read.tex, 0)
    pass(p, divergence)

    p = progs.clear
    gl.useProgram(p.p)
    gl.uniform1f(p.uniforms.value, CONF.pressureDecay)
    bindTex(p, 'uTexture', pressure.read.tex, 0)
    pass(p, pressure.write)
    pressure.swap()

    p = progs.pressure
    gl.useProgram(p.p)
    gl.uniform2f(p.uniforms.texelSize, tx, ty)
    bindTex(p, 'uDivergence', divergence.tex, 1)
    for (let i = 0; i < CONF.pressureIterations; i++) {
      bindTex(p, 'uPressure', pressure.read.tex, 0)
      pass(p, pressure.write)
      pressure.swap()
    }

    p = progs.gradientSubtract
    gl.useProgram(p.p)
    gl.uniform2f(p.uniforms.texelSize, tx, ty)
    bindTex(p, 'uPressure', pressure.read.tex, 0)
    bindTex(p, 'uVelocity', velocity.read.tex, 1)
    pass(p, velocity.write)
    velocity.swap()

    p = progs.advection
    gl.useProgram(p.p)
    gl.uniform2f(p.uniforms.texelSize, tx, ty)
    gl.uniform1f(p.uniforms.dt, dt)
    gl.uniform1f(p.uniforms.dissipation, CONF.velocityDissipation)
    bindTex(p, 'uVelocity', velocity.read.tex, 0)
    bindTex(p, 'uSource', velocity.read.tex, 1)
    pass(p, velocity.write)
    velocity.swap()

    gl.useProgram(p.p)
    gl.uniform2f(p.uniforms.texelSize, 1 / dyeSize.w, 1 / dyeSize.h)
    gl.uniform1f(p.uniforms.dissipation, CONF.dyeDissipation)
    bindTex(p, 'uVelocity', velocity.read.tex, 0)
    bindTex(p, 'uSource', dye.read.tex, 1)
    pass(p, dye.write)
    dye.swap()
  }

  function render() {
    const p = progs.display
    gl.useProgram(p.p)
    bindTex(p, 'uTexture', dye.read.tex, 0)
    gl.enable(gl.BLEND)
    gl.blendFunc(gl.SRC_ALPHA, gl.ONE_MINUS_SRC_ALPHA)
    pass(p, null)
    gl.disable(gl.BLEND)
  }

  // 双 Lissajous 自动 emitter: 不动鼠标背景也在缓慢涌动
  const emitters = [
    { a: 0.31, b: 0.23, pa: 0, pb: 1.2, rx: 0.34, ry: 0.26, next: 0, ci: 0 },
    { a: 0.19, b: 0.29, pa: 2.1, pb: 0.4, rx: 0.38, ry: 0.3, next: 900, ci: 1 },
  ]
  function emitterPos(e, t) {
    return {
      x: 0.5 + e.rx * Math.sin(e.a * t + e.pa),
      y: 0.5 + e.ry * Math.sin(e.b * t + e.pb),
      vx: e.rx * e.a * Math.cos(e.a * t + e.pa),
      vy: e.ry * e.b * Math.cos(e.b * t + e.pb),
    }
  }

  const pointer = { x: 0, y: 0, dx: 0, dy: 0, has: false, moved: false }

  let raf = 0
  let running = true
  let destroyed = false
  let last = performance.now()
  let elapsed = 0
  // 保险丝: 跳过前 30 帧预热, 采样 90 帧平均帧时 >25ms 自毁回退
  let fuseFrames = 0
  let fuseTotal = 0
  let fuseDone = false
  let liveNotified = false

  function frame(now) {
    if (destroyed || !running) return
    const dtMs = now - last
    const dt = Math.min(dtMs / 1000, 0.016)
    last = now
    elapsed += dt

    if (!fuseDone) {
      fuseFrames++
      if (fuseFrames > 30) fuseTotal += dtMs
      if (fuseFrames >= 120) {
        fuseDone = true
        if (fuseTotal / 90 > 25) {
          destroy()
          if (onFallback) onFallback()
          return
        }
      }
    }

    for (const e of emitters) {
      if (now >= e.next) {
        e.next = now + 1500 + Math.random() * 1000
        const p = emitterPos(e, elapsed * 0.35)
        const c = BRAND[e.ci % 3]
        e.ci++
        splat(p.x, p.y, p.vx * 6, p.vy * 6, [c[0] * 0.34, c[1] * 0.34, c[2] * 0.34], 0.009)
      }
    }
    if (pointer.moved) {
      pointer.moved = false
      // 拖尾色随位置在蓝紫间过渡, 避免整条单色
      const t = Math.min(Math.max(pointer.x, 0), 1)
      const a = BRAND[0], b = BRAND[1]
      const mix = [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t, a[2] + (b[2] - a[2]) * t]
      splat(pointer.x, pointer.y, pointer.dx * 24, pointer.dy * 24, [mix[0] * 0.16, mix[1] * 0.16, mix[2] * 0.16], 0.0035)
    }

    simulate(dt)
    render()

    if (!liveNotified && fuseFrames > 3) {
      liveNotified = true
      if (onLive) onLive()
    }
    raf = requestAnimationFrame(frame)
  }
  raf = requestAnimationFrame(t => { last = t; frame(t) })

  let resizeTimer = 0
  function onResize() {
    clearTimeout(resizeTimer)
    resizeTimer = setTimeout(() => {
      if (destroyed) return
      sizeCanvas()
      freeTargets()
      allocTargets()
    }, 200)
  }
  window.addEventListener('resize', onResize, { passive: true })

  function onVisibility() {
    if (destroyed) return
    if (document.hidden) {
      running = false
      cancelAnimationFrame(raf)
    } else if (!running) {
      running = true
      raf = requestAnimationFrame(t => { last = t; frame(t) })
    }
  }
  document.addEventListener('visibilitychange', onVisibility)

  function destroy() {
    if (destroyed) return
    destroyed = true
    cancelAnimationFrame(raf)
    clearTimeout(resizeTimer)
    window.removeEventListener('resize', onResize)
    document.removeEventListener('visibilitychange', onVisibility)
    freeTargets()
    gl.deleteBuffer(quad)
    for (const k in progs) gl.deleteProgram(progs[k].p)
    gl.deleteShader(vs)
    const lose = gl.getExtension('WEBGL_lose_context')
    if (lose) lose.loseContext()
  }

  return {
    destroy,
    // clientX/clientY 像素坐标 → UV(y 向上); dx/dy 为本帧位移(px)
    pointer(cx, cy, dx, dy) {
      if (destroyed) return
      const w = canvas.clientWidth || 1
      const h = canvas.clientHeight || 1
      pointer.x = cx / w
      pointer.y = 1 - cy / h
      pointer.dx = dx / w
      pointer.dy = -dy / h
      pointer.moved = true
    },
  }
}
