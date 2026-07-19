<template>
  <div class="home" ref="root">
    <!-- SVG noise filter (used by .grain) -->
    <svg class="noise-svg" aria-hidden="true">
      <filter id="grain">
        <feTurbulence type="fractalNoise" baseFrequency="0.9" numOctaves="2" stitchTiles="stitch"/>
        <feColorMatrix values="0 0 0 0 0  0 0 0 0 0  0 0 0 0 0  0 0 0 0.6 0"/>
      </filter>
    </svg>

    <!-- Custom cursor -->
    <div class="cursor-dot" ref="cursorDot"></div>
    <div class="cursor-ring" ref="cursorRing"></div>

    <!-- Backdrop layers -->
    <div class="bg">
      <div class="grid"></div>
      <div class="beam beam-1"></div>
      <div class="beam beam-2"></div>
      <div class="orb orb-1"></div>
      <div class="orb orb-2"></div>
      <div class="grain"></div>
    </div>

    <!-- ─── Hero ─── -->
    <section class="hero">
      <div class="wrap">
        <div class="badge" data-reveal>
          <span class="badge-dot"></span>
          <span class="badge-text">v0.2.12 — MIT Licensed</span>
          <span class="badge-arrow">→</span>
        </div>

        <h1 class="headline">
          <span class="line" data-reveal><span class="fill">Windows 开发环境</span></span>
          <span class="line" data-reveal style="--d:.08s">
            <span class="fill gradient">从混沌</span><span class="fill">，到</span><span class="fill gradient">秩序</span><span class="fill dot">.</span>
          </span>
        </h1>

        <p class="sub" data-reveal style="--d:.22s">
          一条命令，装好 20+ 开发工具，配好环境变量。<br class="md-only">
          告别手动下载、PATH 冲突、注册表污染。
        </p>

        <div class="install-row" data-reveal style="--d:.32s">
          <div class="install" :class="{ copied }" @click="copy" data-magnet>
            <span class="i-gradient"></span>
            <span class="i-prompt">PS</span>
            <code class="i-cmd">irm hudo.zexa.cc/install.ps1 | iex</code>
            <span class="i-action">
              <svg v-if="!copied" width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="9" y="9" width="13" height="13" rx="2"/><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"/></svg>
              <svg v-else width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="#10b981" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><polyline points="20 6 9 17 4 12"/></svg>
              <span class="i-label">{{ copied ? 'Copied' : 'Copy' }}</span>
            </span>
          </div>
        </div>

        <div class="cta-row" data-reveal style="--d:.42s">
          <a href="/guide/quickstart" class="btn primary" data-magnet>
            <span>开始使用</span>
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><line x1="5" y1="12" x2="19" y2="12"/><polyline points="12 5 19 12 12 19"/></svg>
          </a>
          <a href="https://github.com/zexadev/hudo" target="_blank" class="btn ghost" data-magnet>
            <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor"><path d="M12 0C5.37 0 0 5.37 0 12c0 5.31 3.435 9.795 8.205 11.385.6.105.825-.255.825-.57 0-.285-.015-1.23-.015-2.235-3.015.555-3.795-.735-4.035-1.41-.135-.345-.72-1.41-1.23-1.695-.42-.225-1.02-.78-.015-.795.945-.015 1.62.87 1.845 1.23 1.08 1.815 2.805 1.305 3.495.99.105-.78.42-1.305.765-1.605-2.67-.3-5.46-1.335-5.46-5.925 0-1.305.465-2.385 1.23-3.225-.12-.3-.54-1.53.12-3.18 0 0 1.005-.315 3.3 1.23.96-.27 1.98-.405 3-.405s2.04.135 3 .405c2.295-1.56 3.3-1.23 3.3-1.23.66 1.65.24 2.88.12 3.18.765.84 1.23 1.905 1.23 3.225 0 4.605-2.805 5.625-5.475 5.925.435.375.81 1.095.81 2.22 0 1.605-.015 2.895-.015 3.3 0 .315.225.69.825.57A12.02 12.02 0 0 0 24 12c0-6.63-5.37-12-12-12z"/></svg>
            <span>GitHub</span>
            <span class="star-count">★ Star</span>
          </a>
        </div>

        <div class="scroll-hint" data-reveal style="--d:.7s">
          <span class="scroll-line"></span>
          <span>SCROLL</span>
        </div>
      </div>
    </section>

    <!-- ─── Marquee ─── -->
    <div class="marquee" aria-hidden="true">
      <div class="mq-track">
        <span v-for="(w, i) in marqueeWords.concat(marqueeWords)" :key="i" class="mq-item">
          <span class="mq-bullet">✦</span>{{ w }}
        </span>
      </div>
    </div>

    <!-- ─── Tools ─── -->
    <section class="sec tools">
      <div class="wrap">
        <div class="sec-head">
          <span class="sec-label" data-reveal>// 01  支持工具</span>
          <h2 class="sec-title" data-reveal style="--d:.08s">
            覆盖你的<span class="gradient">完整工具链</span>
          </h2>
          <p class="sec-desc" data-reveal style="--d:.16s">从 Git 到 Redis，20 个开发工具一站式管理。</p>
        </div>

        <div class="tools-grid" data-reveal-group>
          <div
            class="tool"
            v-for="(t, i) in tools"
            :key="t.name"
            :style="`--i:${i}`"
            @mousemove="onTilt($event)"
            @mouseleave="offTilt($event)"
          >
            <div class="tool-inner">
              <div class="tool-glow"></div>
              <div class="tool-svg" v-html="t.svg"></div>
              <span class="tool-name">{{ t.name }}</span>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- ─── Features ─── -->
    <section class="sec features">
      <div class="wrap">
        <div class="sec-head">
          <span class="sec-label" data-reveal>// 02  核心特性</span>
          <h2 class="sec-title" data-reveal style="--d:.08s">为 Windows 开发者<span class="gradient">重新设计</span></h2>
        </div>

        <div class="feat-grid">
          <div
            class="feat"
            v-for="f in features"
            :key="f.title"
            data-reveal
            @mousemove="onSpot($event)"
          >
            <div class="feat-border"></div>
            <div class="feat-spot"></div>
            <div class="feat-body">
              <div class="feat-icon" v-html="f.icon"></div>
              <h3 class="feat-title">{{ f.title }}</h3>
              <p class="feat-desc">{{ f.desc }}</p>
              <div class="feat-arrow">
                <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><line x1="5" y1="12" x2="19" y2="12"/><polyline points="12 5 19 12 12 19"/></svg>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- ─── Terminal Demo ─── -->
    <section class="sec demo">
      <div class="wrap">
        <div class="sec-head">
          <span class="sec-label" data-reveal>// 03  实际效果</span>
          <h2 class="sec-title" data-reveal style="--d:.08s">交互式<span class="gradient">工具选择</span></h2>
        </div>

        <div class="term-wrap" data-reveal ref="termWrap">
          <div class="term-glow"></div>
          <div class="term">
            <div class="term-bar">
              <div class="term-dots"><span/><span/><span/></div>
              <div class="term-title">Administrator: PowerShell — hudo</div>
              <div class="term-spacer"/>
            </div>
            <div class="term-body">
              <div v-for="(l, i) in termLines.slice(0, termVisible)" :key="i" class="tl" :class="l.k">
                <span v-if="l.k==='cmd'" class="tl-prompt">PS&nbsp;C:\&gt;</span>
                <span class="tl-text">{{ l.t }}</span>
                <span v-if="i===termVisible-1 && typing" class="caret">▍</span>
              </div>
              <div v-if="termVisible >= termLines.length" class="tl-bar">
                <div class="tl-bar-track"><div class="tl-bar-fill"></div></div>
                <span class="tl-bar-text">Installing Rust 1.84.0 · 68%</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- ─── CTA ─── -->
    <section class="sec cta-sec">
      <div class="wrap">
        <div class="cta" data-reveal>
          <div class="cta-beam"></div>
          <div class="cta-grid"></div>
          <p class="cta-label">准备好了吗?</p>
          <h2 class="cta-title">一条命令,<br>开启你的 Windows 开发之旅.</h2>
          <div class="cta-btns">
            <a href="/guide/quickstart" class="btn primary lg" data-magnet>
              阅读文档
              <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><line x1="5" y1="12" x2="19" y2="12"/><polyline points="12 5 19 12 12 19"/></svg>
            </a>
            <a href="/tools/" class="btn outline lg" data-magnet>浏览工具列表</a>
          </div>
        </div>
      </div>
    </section>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted, nextTick } from 'vue'

const root = ref(null)
const cursorDot = ref(null)
const cursorRing = ref(null)
const termWrap = ref(null)

const copied = ref(false)
function copy() {
  navigator.clipboard.writeText('irm hudo.zexa.cc/install.ps1 | iex')
  copied.value = true
  setTimeout(() => { copied.value = false }, 1800)
}

const marqueeWords = ['Git', 'GitHub CLI', 'Node.js', 'Bun', 'Rust', 'Go', 'JDK', 'Maven', 'Gradle', 'Python (uv)', 'Miniconda', 'MySQL', 'Redis', 'PostgreSQL', 'VS Code', 'PyCharm', 'MinGW', 'Chrome', 'Claude Code']

const tools = [
  { name: 'Git', svg: `<svg viewBox="0 0 24 24" fill="#F05032"><path d="M23.546 10.93L13.067.452a1.55 1.55 0 0 0-2.188 0L8.708 2.627l2.76 2.76a1.838 1.838 0 0 1 2.327 2.341l2.66 2.66a1.838 1.838 0 1 1-1.103 1.06l-2.48-2.48v6.53a1.838 1.838 0 1 1-1.513-.122V8.78a1.838 1.838 0 0 1-.998-2.41L7.629 3.64.452 10.816a1.55 1.55 0 0 0 0 2.188l10.48 10.48a1.55 1.55 0 0 0 2.186 0l10.43-10.43a1.55 1.55 0 0 0 0-2.123z"/></svg>` },
  { name: 'GitHub CLI', svg: `<svg viewBox="0 0 24 24" fill="currentColor"><path d="M12 0C5.37 0 0 5.37 0 12c0 5.31 3.435 9.795 8.205 11.385.6.105.825-.255.825-.57 0-.285-.015-1.23-.015-2.235-3.015.555-3.795-.735-4.035-1.41-.135-.345-.72-1.41-1.23-1.695-.42-.225-1.02-.78-.015-.795.945-.015 1.62.87 1.845 1.23 1.08 1.815 2.805 1.305 3.495.99.105-.78.42-1.305.765-1.605-2.67-.3-5.46-1.335-5.46-5.925 0-1.305.465-2.385 1.23-3.225-.12-.3-.54-1.53.12-3.18 0 0 1.005-.315 3.3 1.23.96-.27 1.98-.405 3-.405s2.04.135 3 .405c2.295-1.56 3.3-1.23 3.3-1.23.66 1.65.24 2.88.12 3.18.765.84 1.23 1.905 1.23 3.225 0 4.605-2.805 5.625-5.475 5.925.435.375.81 1.095.81 2.22 0 1.605-.015 2.895-.015 3.3 0 .315.225.69.825.57A12.02 12.02 0 0 0 24 12c0-6.63-5.37-12-12-12z"/></svg>` },
  { name: 'Node.js', svg: `<svg viewBox="0 0 24 24" fill="#339933"><path d="M11.998 24c-.321 0-.641-.084-.922-.247l-2.936-1.737c-.438-.245-.224-.332-.08-.383.585-.203.703-.25 1.328-.604.065-.037.151-.023.218.017l2.256 1.339a.29.29 0 0 0 .272 0l8.795-5.076a.277.277 0 0 0 .134-.238V6.921a.28.28 0 0 0-.137-.242l-8.791-5.072a.278.278 0 0 0-.271 0L3.075 6.68a.28.28 0 0 0-.138.24v10.15c0 .099.053.19.137.24l2.409 1.392c1.307.654 2.108-.116 2.108-.89V7.787c0-.142.114-.253.256-.253h1.115c.139 0 .255.112.255.253v10.021c0 1.745-.95 2.745-2.604 2.745-.508 0-.909 0-2.026-.551L2.28 18.675A1.857 1.857 0 0 1 1.36 17.07V6.921c0-.645.344-1.248.92-1.572L11.075.273a1.946 1.946 0 0 1 1.846 0l8.794 5.076c.576.324.92.927.92 1.572v10.15a1.86 1.86 0 0 1-.92 1.604l-8.795 5.078a1.834 1.834 0 0 1-.922.247z"/></svg>` },
  { name: 'Bun', svg: `<svg viewBox="0 0 24 24" fill="#FBF0DF"><path d="M12 22C6.477 22 2 17.523 2 12S6.477 2 12 2s10 4.477 10 10-4.477 10-10 10z"/><path d="M8.5 10.5a1.5 1.5 0 1 0 0-3 1.5 1.5 0 0 0 0 3zm7 0a1.5 1.5 0 1 0 0-3 1.5 1.5 0 0 0 0 3zm-7.5 3s1.5 3 4 3 4-3 4-3" fill="none" stroke="#000" stroke-width="1.5" stroke-linecap="round"/></svg>` },
  { name: 'Rust', svg: `<svg viewBox="0 0 24 24" fill="currentColor"><path d="M23.835 11.703l-1.008-.622-.093-.3.695-.88a.253.253 0 0 0-.031-.32l-.665-.66a.253.253 0 0 0-.318-.034l-.893.676-.3-.085-.554-1.03a.253.253 0 0 0-.276-.14l-.937.135a.253.253 0 0 0-.213.226l-.052 1.147-.268.164-.985-.468a.253.253 0 0 0-.303.072l-.635.783a.253.253 0 0 0-.014.318l.63.914-.155.285-1.107.195a.253.253 0 0 0-.208.212l-.14.934a.253.253 0 0 0 .138.28l1.023.543.04.306-.804.81a.253.253 0 0 0-.042.316l.482.85c.065.115.2.172.325.14l1.13-.25.208.24-.347 1.088a.253.253 0 0 0 .104.295l.832.524a.253.253 0 0 0 .32-.022l.818-.803.288.064.117 1.143a.253.253 0 0 0 .228.225l.968.082a.253.253 0 0 0 .268-.164l.395-1.083.296-.06.68.915a.253.253 0 0 0 .31.086l.9-.383a.253.253 0 0 0 .162-.293l-.256-1.128.235-.193 1.05.504c.12.058.265.017.337-.093l.534-.82a.253.253 0 0 0-.018-.3l-.726-.874.11-.286 1.15-.085c.131-.01.24-.103.26-.232l.155-.956a.253.253 0 0 0-.157-.278zM12.04 17.762a5.783 5.783 0 1 1 0-11.566 5.783 5.783 0 0 1 0 11.566z"/></svg>` },
  { name: 'Go', svg: `<svg viewBox="0 0 24 24" fill="#00ADD8"><path d="M1.811 10.231c-.047 0-.058-.023-.035-.059l.246-.315c.023-.035.081-.058.128-.058h4.172c.046 0 .058.035.035.07l-.199.303c-.023.036-.082.07-.117.07zM.047 11.306c-.047 0-.059-.023-.035-.058l.245-.316c.023-.035.082-.058.129-.058h5.328c.047 0 .07.035.058.07l-.093.28c-.012.047-.058.07-.105.07zm2.828 1.075c-.047 0-.059-.035-.035-.07l.163-.292c.023-.035.07-.07.117-.07h2.337c.047 0 .07.035.07.082l-.023.28c0 .047-.047.082-.082.082z"/></svg>` },
  { name: 'JDK', svg: `<svg viewBox="0 0 24 24" fill="#ED8B00"><path d="M8.851 18.56s-.917.534.653.714c1.902.218 2.874.187 4.969-.211 0 0 .552.346 1.321.646-4.699 2.013-10.633-.118-6.943-1.149m-.575-2.627s-1.028.762.542.924c2.032.209 3.636.227 6.413-.308 0 0 .384.389.987.602-5.679 1.661-12.007.13-7.942-1.218m4.84-4.458c1.158 1.333-.304 2.533-.304 2.533s2.939-1.518 1.589-3.418c-1.261-1.772-2.228-2.652 3.007-5.688 0 0-8.216 2.051-4.292 6.573m6.924 10.083s.679.559-.747.991c-2.712.822-11.288 1.069-13.669.033-.856-.373.75-.89 1.254-.998.527-.114.828-.093.828-.093-.953-.671-6.156 1.317-2.643 1.887 9.58 1.553 17.462-.7 14.977-1.82M9.292 13.21s-4.362 1.036-1.544 1.412c1.189.159 3.561.123 5.77-.062 1.806-.152 3.618-.477 3.618-.477s-.637.272-1.098.587c-4.429 1.165-12.986.623-10.522-.568 2.082-1.006 3.776-.892 3.776-.892m7.824 4.374c4.503-2.34 2.421-4.589.968-4.285-.355.074-.515.138-.515.138s.132-.207.385-.297c2.875-1.011 5.086 2.981-.928 4.562 0 0 .07-.062.09-.118M14.401 0s2.494 2.494-2.365 6.33c-3.896 3.077-.888 4.832 0 6.836-2.274-2.053-3.943-3.858-2.824-5.539 1.644-2.469 6.197-3.665 5.189-7.627"/></svg>` },
  { name: 'Maven', svg: `<svg viewBox="0 0 24 24" fill="#C71A36"><path d="M4.711 2.876L.43 10.124l4.28 7.248h2.07L2.5 10.124l4.28-7.248zm7.85 0L8.283 10.124l4.278 7.248h2.07l-4.278-7.248 4.278-7.248zm7.848 0l-4.278 7.248 4.278 7.248h2.07l-4.28-7.248 4.28-7.248z"/></svg>` },
  { name: 'Gradle', svg: `<svg viewBox="0 0 24 24" fill="#02303A"><path d="M22.395 5.2a3.16 3.16 0 0 0-4.473 0c-.148.148-.3.39-.3.39L12.97 1.54s-2.132-.906-3.723.39L7.51.192 6.433 1.343l1.64 1.64s-.597.745-.894 1.342a4.42 4.42 0 0 0-.447 2.236 3.158 3.158 0 0 0 3.13 3.13z"/></svg>` },
  { name: 'Python (uv)', svg: `<svg viewBox="0 0 24 24" fill="none"><path d="M9.585 11.692h4.328s2.432.039 2.432-2.35V5.391S16.714 3 12.304 3h-1.63C7.04 3 5.654 5.196 5.654 5.196v2.828h4.766v.706H5.654S3 8.438 3 12.688s2.317 4.063 2.317 4.063h1.384v-1.955s-.074-2.317 2.28-2.317z" fill="#366C9C"/><circle cx="8.35" cy="5.87" r=".78" fill="#FFC331"/><path d="M14.415 12.308h-4.328s-2.432-.039-2.432 2.35v3.951S7.286 21 11.696 21h1.63c3.634 0 5.02-2.196 5.02-2.196v-2.828h-4.766v-.706h4.766S21 15.562 21 11.312s-2.317-4.063-2.317-4.063h-1.384v1.955s.074 2.317-2.28 2.317z" fill="#FFC331"/><circle cx="15.65" cy="18.13" r=".78" fill="#366C9C"/></svg>` },
  { name: 'Miniconda', svg: `<svg viewBox="0 0 24 24" fill="#44A833"><circle cx="12" cy="12" r="10"/></svg>` },
  { name: 'MySQL', svg: `<svg viewBox="0 0 24 24" fill="#4479A1"><path d="M5.17 7.59c-.15 0-.24.02-.35.04v.01h.01c.07.13.18.22.27.33l.19.4.01-.01c.12-.08.18-.22.18-.43-.05-.05-.06-.12-.1-.17-.05-.07-.16-.11-.21-.17zm11.26-.26c-.47.01-.84.07-1.16.18-.09.03-.24.03-.25.15.05.05.06.12.1.18.08.13.21.3.33.39.13.1.26.21.4.3.25.16.53.25.77.4.14.09.28.21.42.31.07.05.12.13.21.16v-.02c-.05-.06-.06-.15-.11-.22-.07-.07-.14-.13-.21-.2-.21-.25-.46-.47-.73-.65-.22-.15-.7-.35-.79-.6z"/></svg>` },
  { name: 'Redis', svg: `<svg viewBox="0 0 24 24" fill="#DC382D"><path d="M10.5 2.661l.54.997-1.797.644 2.409.218.748 1.246.467-1.135 2.025-.22-1.688-.76.604-1.203-1.89.674zm6.727 2.08l-3.167 1.345.87 1.715-2.894-.312 1.737 1.143-.626 1.26 2.536-.93c.7.678 1.103 1.08 1.674 1.635-.558.173-1.088.335-1.65.525.08.38.155.74.228 1.104-.467-.27-.935-.532-1.39-.803l-1.565.604.466-1.103-1.202-.82 1.584-.087.387-.914 2.38.274-.86-1.675zm-9.62 4.167c2.567 0 4.648.683 4.648 1.525 0 .842-2.081 1.525-4.648 1.525S2.96 11.275 2.96 10.433c0-.842 2.08-1.525 4.647-1.525zm0 5.999c2.567 0 4.648.682 4.648 1.524v2.17c0 .843-2.081 1.526-4.648 1.526S2.96 18.926 2.96 18.083v-2.17c0-.841 2.08-1.524 4.647-1.524z"/></svg>` },
  { name: 'PostgreSQL', svg: `<svg viewBox="0 0 24 24" fill="#4169E1"><path d="M17.128 0a10.134 10.134 0 0 0-2.755.403l-.063.02A10.922 10.922 0 0 0 12.6.258C11.422.238 10.4.524 9.594 1.01 8.97.753 7.782.397 6.473.427c-1.527.035-3.078.623-4.08 2.058-.97 1.39-1.425 3.466-1.18 6.14.073.794.207 1.618.395 2.463.277 1.243.675 2.529 1.182 3.725.498 1.176 1.09 2.22 1.838 2.982.372.379.836.72 1.382.856.547.136 1.187.062 1.687-.306.376-.276.618-.673.75-.984z"/></svg>` },
  { name: 'VS Code', svg: `<svg viewBox="0 0 24 24" fill="#007ACC"><path d="M17.583 2.29L9.87 9.176 5.794 5.983 4.4 6.61v10.78l1.394.627 4.076-3.193 7.713 6.886L21.6 19.77V4.23zM5.882 14.538V9.462L8.57 12zm7.082.907L9.87 12.9v-1.8l3.094-2.544v6.889z"/></svg>` },
  { name: 'PyCharm', svg: `<svg viewBox="0 0 24 24" fill="currentColor"><rect x="3" y="3" width="18" height="18" rx="2"/></svg>` },
  { name: 'MinGW', svg: `<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><rect x="3" y="4" width="18" height="16" rx="2"/><path d="M7 15l3-3-3-3m5 6h4"/></svg>` },
  { name: 'Chrome', svg: `<svg viewBox="0 0 24 24" fill="none"><circle cx="12" cy="12" r="10" fill="#4285F4"/><circle cx="12" cy="12" r="4.5" fill="#fff"/><circle cx="12" cy="12" r="3" fill="#4285F4"/><path d="M12 7.5h9.5" stroke="#EA4335" stroke-width="5" opacity=".9"/><path d="M7.25 14.75L2.5 6" stroke="#34A853" stroke-width="5" opacity=".9"/><path d="M7.25 14.75l4.75-2" stroke="#FBBC05" stroke-width="5" opacity=".9"/></svg>` },
  { name: 'Claude Code', svg: `<svg viewBox="0 0 24 24" fill="none"><path d="M12 2l2.4 7.2H22l-6 4.8 2.4 7.2L12 16.4 5.6 21.2 8 14 2 9.2h7.6z" fill="#D97706" opacity=".9"/></svg>` },
]

const features = [
  { title: '秒级安装', desc: '一条命令完成 hudo 自身安装,交互菜单勾选工具,全程自动下载/解压/配环境变量。', icon: `<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.6"><path d="M13 2L3 14h9l-1 8 10-12h-9l1-8z"/></svg>` },
  { title: '零污染隔离', desc: '所有工具统一安装到独立目录,不写注册表、不污染系统 PATH,卸载干净无残留。', icon: `<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.6"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/></svg>` },
  { title: '版本管理', desc: '自动检测最新版本,支持指定版本号。内置 GitHub API 与官方源双通道查询。', icon: `<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.6"><path d="M6 3v12"/><circle cx="18" cy="6" r="3"/><circle cx="6" cy="18" r="3"/><path d="M18 9a9 9 0 0 1-9 9"/></svg>` },
  { title: '镜像加速', desc: '内置国内镜像源,GitHub 下载自动走代理。大陆用户也能流畅安装。', icon: `<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.6"><circle cx="12" cy="12" r="10"/><line x1="2" y1="12" x2="22" y2="12"/><path d="M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z"/></svg>` },
  { title: '配置档案', desc: '导出当前环境为 profile 文件,新机器一键还原。团队统一开发环境不再是难题。', icon: `<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.6"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8"/></svg>` },
  { title: '开源免费', desc: '基于 MIT 协议开源,Rust 编写,单文件二进制分发。社区驱动,欢迎贡献。', icon: `<svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.6"><path d="M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z"/></svg>` },
]

// ─── Terminal typewriter ───
const termLines = ref([
  { k: 'cmd', t: 'hudo' },
  { k: 'dim', t: 'hudo v0.2.12 — The Chaos Bootstrapper' },
  { k: 'txt', t: 'Select tools to install:' },
  { k: 'on',  t: '◉  Git            2.47.1' },
  { k: 'on',  t: '◉  Node.js        22.13.0' },
  { k: 'on',  t: '◉  Rust           1.84.0' },
  { k: 'off', t: '○  JDK            21.0.5' },
  { k: 'on',  t: '◉  VS Code        1.96.4' },
  { k: 'on',  t: '◉  Python (uv)    0.5.15' },
])
const termVisible = ref(0)
const typing = ref(false)
let termTimer = null
function runTerm() {
  if (termVisible.value > 0) return
  typing.value = true
  const step = () => {
    if (termVisible.value < termLines.value.length) {
      termVisible.value++
      termTimer = setTimeout(step, 180)
    } else {
      typing.value = false
    }
  }
  step()
}

// ─── Tilt on tool cards ───
function onTilt(e) {
  const el = e.currentTarget
  const r = el.getBoundingClientRect()
  const x = (e.clientX - r.left) / r.width - 0.5
  const y = (e.clientY - r.top) / r.height - 0.5
  el.style.setProperty('--rx', `${(-y * 8).toFixed(2)}deg`)
  el.style.setProperty('--ry', `${(x * 10).toFixed(2)}deg`)
  el.style.setProperty('--mx', `${(e.clientX - r.left).toFixed(0)}px`)
  el.style.setProperty('--my', `${(e.clientY - r.top).toFixed(0)}px`)
}
function offTilt(e) {
  const el = e.currentTarget
  el.style.setProperty('--rx', '0deg')
  el.style.setProperty('--ry', '0deg')
}

// ─── Spotlight on feature cards ───
function onSpot(e) {
  const el = e.currentTarget
  const r = el.getBoundingClientRect()
  el.style.setProperty('--mx', `${e.clientX - r.left}px`)
  el.style.setProperty('--my', `${e.clientY - r.top}px`)
}

// ─── Custom cursor + magnet ───
let cx = 0, cy = 0, rx = 0, ry = 0, rafId = 0
function moveCursor(e) {
  cx = e.clientX; cy = e.clientY
  if (cursorDot.value) {
    cursorDot.value.style.transform = `translate3d(${cx}px, ${cy}px, 0)`
  }
}
function tickRing() {
  rx += (cx - rx) * 0.18
  ry += (cy - ry) * 0.18
  if (cursorRing.value) {
    cursorRing.value.style.transform = `translate3d(${rx}px, ${ry}px, 0)`
  }
  rafId = requestAnimationFrame(tickRing)
}

let io = null
onMounted(async () => {
  await nextTick()

  // cursor (desktop only)
  const canHover = window.matchMedia('(hover: hover)').matches
  if (canHover) {
    document.body.classList.add('cursor-on')
    window.addEventListener('mousemove', moveCursor, { passive: true })
    tickRing()
    // magnet hover
    document.querySelectorAll('[data-magnet]').forEach(el => {
      el.addEventListener('mouseenter', () => document.body.classList.add('cursor-hover'))
      el.addEventListener('mouseleave', () => document.body.classList.remove('cursor-hover'))
    })
  }

  // scroll reveal
  io = new IntersectionObserver((entries) => {
    entries.forEach(en => {
      if (en.isIntersecting) {
        en.target.classList.add('in')
        if (en.target.hasAttribute('data-reveal-group')) {
          en.target.querySelectorAll(':scope > *').forEach((c, i) => {
            c.style.setProperty('--d', `${i * 0.04}s`)
            c.classList.add('in')
          })
        }
        // terminal trigger
        if (en.target === termWrap.value) runTerm()
        io.unobserve(en.target)
      }
    })
  }, { rootMargin: '0px 0px -10% 0px', threshold: 0.12 })

  document.querySelectorAll('[data-reveal], [data-reveal-group]').forEach(el => io.observe(el))
})

onUnmounted(() => {
  window.removeEventListener('mousemove', moveCursor)
  if (rafId) cancelAnimationFrame(rafId)
  if (io) io.disconnect()
  if (termTimer) clearTimeout(termTimer)
  document.body.classList.remove('cursor-on', 'cursor-hover')
})
</script>

<style scoped>
/* ───────────── root ───────────── */
.home {
  position: relative;
  color: var(--vp-c-text-1);
  font-feature-settings: "ss01", "ss02", "cv11";
  --brand: #60a5fa;
  --brand-2: #a78bfa;
  --brand-3: #f472b6;
  --line: rgba(255,255,255,.06);
  --text-dim: rgba(255,255,255,.55);
}
:root:not(.dark) .home {
  --line: rgba(15,23,42,.08);
  --text-dim: rgba(15,23,42,.6);
}

.wrap {
  max-width: 1200px;
  margin: 0 auto;
  padding: 0 32px;
  position: relative;
}

.noise-svg { position: absolute; width: 0; height: 0; }

/* ───────────── backdrop ───────────── */
.bg {
  position: fixed;
  inset: 0;
  pointer-events: none;
  overflow: hidden;
  z-index: 0;
}

.grid {
  position: absolute;
  inset: -2px;
  background-image:
    linear-gradient(var(--line) 1px, transparent 1px),
    linear-gradient(90deg, var(--line) 1px, transparent 1px);
  background-size: 64px 64px;
  mask-image: radial-gradient(ellipse 80% 60% at 50% 40%, #000 30%, transparent 80%);
  -webkit-mask-image: radial-gradient(ellipse 80% 60% at 50% 40%, #000 30%, transparent 80%);
}

.beam {
  position: absolute;
  width: 60vw;
  aspect-ratio: 1;
  border-radius: 50%;
  filter: blur(100px);
  opacity: .6;
  animation: drift 22s ease-in-out infinite alternate;
}
.beam-1 {
  top: -20%;
  left: -10%;
  background: radial-gradient(closest-side, rgba(96,165,250,.28), transparent);
}
.beam-2 {
  bottom: -30%;
  right: -10%;
  background: radial-gradient(closest-side, rgba(167,139,250,.24), transparent);
  animation-delay: -11s;
}
:root:not(.dark) .beam-1 { background: radial-gradient(closest-side, rgba(59,130,246,.18), transparent); }
:root:not(.dark) .beam-2 { background: radial-gradient(closest-side, rgba(147,51,234,.14), transparent); }

@keyframes drift {
  0%   { transform: translate(0, 0) scale(1); }
  50%  { transform: translate(4vw, 3vh) scale(1.1); }
  100% { transform: translate(-3vw, -2vh) scale(.95); }
}

.orb {
  position: absolute;
  width: 8px; height: 8px;
  border-radius: 50%;
  background: var(--brand);
  box-shadow: 0 0 40px 10px var(--brand);
  opacity: .55;
}
.orb-1 { top: 24%; left: 14%; animation: float 9s ease-in-out infinite; }
.orb-2 { top: 58%; right: 18%; background: var(--brand-2); box-shadow: 0 0 40px 10px var(--brand-2); animation: float 11s ease-in-out infinite reverse; }

@keyframes float {
  0%,100% { transform: translateY(0) }
  50%     { transform: translateY(-24px) }
}

.grain {
  position: absolute;
  inset: -50%;
  opacity: .12;
  pointer-events: none;
  filter: url(#grain) contrast(170%) brightness(1000%);
  mix-blend-mode: overlay;
}
:root:not(.dark) .grain { opacity: .05; mix-blend-mode: multiply; }

/* ───────────── cursor ───────────── */
.cursor-dot, .cursor-ring {
  position: fixed;
  top: 0; left: 0;
  pointer-events: none;
  z-index: 9999;
  opacity: 0;
  will-change: transform;
  transition: opacity .3s;
}
.cursor-on .cursor-dot, .cursor-on .cursor-ring { opacity: 1; }

.cursor-dot {
  width: 6px; height: 6px;
  margin: -3px 0 0 -3px;
  border-radius: 50%;
  background: var(--brand);
  mix-blend-mode: difference;
}
.cursor-ring {
  width: 36px; height: 36px;
  margin: -18px 0 0 -18px;
  border: 1px solid rgba(96,165,250,.55);
  border-radius: 50%;
  transition: width .2s, height .2s, margin .2s, border-color .2s, background .2s;
}
.cursor-hover .cursor-ring {
  width: 58px; height: 58px;
  margin: -29px 0 0 -29px;
  background: rgba(96,165,250,.08);
  border-color: rgba(96,165,250,.8);
}
.cursor-hover .cursor-dot { transform-origin: center; }

/* ───────────── reveal ───────────── */
[data-reveal] {
  opacity: 0;
  transform: translateY(22px);
  transition: opacity .9s cubic-bezier(.2,.7,.1,1) var(--d, 0s),
              transform .9s cubic-bezier(.2,.7,.1,1) var(--d, 0s);
}
[data-reveal].in { opacity: 1; transform: none; }

[data-reveal-group] > * {
  opacity: 0;
  transform: translateY(16px) scale(.98);
  transition: opacity .7s cubic-bezier(.2,.7,.1,1) var(--d, 0s),
              transform .7s cubic-bezier(.2,.7,.1,1) var(--d, 0s);
}
[data-reveal-group] > *.in { opacity: 1; transform: none; }

/* headline line reveal */
.line {
  display: block;
  overflow: hidden;
}
.line .fill {
  display: inline-block;
  transform: translateY(100%);
  transition: transform 1.1s cubic-bezier(.2,.7,.1,1) var(--d, 0s);
}
.line.in .fill { transform: none; }
.line.in .fill:nth-child(2) { transition-delay: calc(var(--d) + .08s); }
.line.in .fill:nth-child(3) { transition-delay: calc(var(--d) + .16s); }
.line.in .fill:nth-child(4) { transition-delay: calc(var(--d) + .24s); }

/* ───────────── hero ───────────── */
.hero {
  position: relative;
  z-index: 1;
  padding: 160px 0 80px;
  min-height: 92vh;
  display: flex;
  align-items: center;
  text-align: center;
}
.hero .wrap { width: 100%; }

.badge {
  display: inline-flex;
  align-items: center;
  gap: 10px;
  padding: 6px 14px 6px 10px;
  margin-bottom: 40px;
  font-size: .78rem;
  font-weight: 500;
  letter-spacing: .01em;
  color: var(--text-dim);
  background: var(--vp-c-bg-soft);
  border: 1px solid var(--line);
  border-radius: 100px;
  backdrop-filter: blur(10px);
}
.badge-dot {
  width: 6px; height: 6px;
  border-radius: 50%;
  background: #10b981;
  box-shadow: 0 0 0 0 rgba(16,185,129,.6);
  animation: pulse 2.2s ease-out infinite;
}
@keyframes pulse {
  0%   { box-shadow: 0 0 0 0 rgba(16,185,129,.6); }
  70%  { box-shadow: 0 0 0 10px rgba(16,185,129,0); }
  100% { box-shadow: 0 0 0 0 rgba(16,185,129,0); }
}
.badge-arrow { color: var(--brand); font-weight: 700; }

.headline {
  font-size: clamp(2.6rem, 7.2vw, 6rem);
  font-weight: 800;
  line-height: .98;
  letter-spacing: -0.045em;
  margin: 0 0 28px;
  color: var(--vp-c-text-1);
}
.gradient {
  background: linear-gradient(135deg, #60a5fa 0%, #a78bfa 50%, #f472b6 100%);
  -webkit-background-clip: text;
  background-clip: text;
  -webkit-text-fill-color: transparent;
}
.dot { color: var(--brand); }

.sub {
  max-width: 620px;
  margin: 0 auto 40px;
  font-size: clamp(.98rem, 1.3vw, 1.15rem);
  line-height: 1.7;
  color: var(--text-dim);
}
.md-only { display: none; }
@media (min-width: 640px) { .md-only { display: inline; } }

/* install box */
.install-row {
  display: flex;
  justify-content: center;
  margin-bottom: 28px;
}
.install {
  position: relative;
  display: inline-flex;
  align-items: center;
  gap: 14px;
  padding: 14px 8px 14px 20px;
  background: var(--vp-c-bg-soft);
  border: 1px solid var(--line);
  border-radius: 14px;
  font-family: 'JetBrains Mono', 'Cascadia Code', monospace;
  font-size: .88rem;
  cursor: pointer;
  overflow: hidden;
  transition: border-color .3s;
}
.install:hover { border-color: rgba(96,165,250,.4); }
.i-gradient {
  position: absolute;
  inset: -1px;
  border-radius: 14px;
  padding: 1px;
  background: conic-gradient(from 180deg, transparent 0%, rgba(96,165,250,.8) 20%, rgba(167,139,250,.7) 40%, rgba(244,114,182,.6) 50%, transparent 70%);
  -webkit-mask: linear-gradient(#fff 0 0) content-box, linear-gradient(#fff 0 0);
  mask: linear-gradient(#fff 0 0) content-box, linear-gradient(#fff 0 0);
  -webkit-mask-composite: xor;
  mask-composite: exclude;
  animation: spin 4s linear infinite;
  opacity: .8;
}
@keyframes spin { to { transform: rotate(1turn); } }

.i-prompt { color: var(--brand-2); font-weight: 700; user-select: none; }
.i-cmd { color: var(--vp-c-text-1); font-weight: 500; }
.i-action {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 6px 10px;
  border-radius: 8px;
  font-size: .75rem;
  font-weight: 600;
  color: var(--text-dim);
  background: var(--vp-c-bg);
  border: 1px solid var(--line);
  transition: all .15s;
}
.install:hover .i-action { color: var(--vp-c-text-1); border-color: var(--text-dim); }
.install.copied .i-action { color: #10b981; border-color: rgba(16,185,129,.4); }
.i-label { font-family: var(--vp-font-family-base); letter-spacing: .02em; }

/* buttons */
.cta-row {
  display: flex;
  justify-content: center;
  gap: 12px;
  flex-wrap: wrap;
}
.btn {
  position: relative;
  display: inline-flex;
  align-items: center;
  gap: 10px;
  padding: 0 22px;
  height: 46px;
  border-radius: 10px;
  font-size: .92rem;
  font-weight: 600;
  letter-spacing: .005em;
  text-decoration: none;
  transition: transform .25s cubic-bezier(.2,.7,.1,1), box-shadow .25s, background .2s, border-color .2s, color .2s;
  overflow: hidden;
  border: 1px solid transparent;
}
.btn.lg { height: 52px; padding: 0 28px; font-size: 1rem; }
.btn:hover { transform: translateY(-1px); }

.btn.primary {
  background: linear-gradient(180deg, #fff, #e8eaef);
  color: #0a0a0f;
}
.dark .btn.primary { background: linear-gradient(180deg, #fff, #d9dde4); }
.btn.primary:hover {
  box-shadow: 0 8px 28px -8px rgba(255,255,255,.3), 0 0 0 1px rgba(255,255,255,.2);
}
:root:not(.dark) .btn.primary {
  background: linear-gradient(180deg, #0f172a, #1e293b);
  color: #fff;
}
:root:not(.dark) .btn.primary:hover {
  box-shadow: 0 8px 28px -8px rgba(15,23,42,.5);
}

.btn.ghost {
  background: var(--vp-c-bg-soft);
  color: var(--vp-c-text-1);
  border-color: var(--line);
}
.btn.ghost:hover { border-color: var(--text-dim); background: var(--vp-c-bg-alt); }
.star-count {
  padding-left: 10px;
  margin-left: 2px;
  border-left: 1px solid var(--line);
  color: var(--text-dim);
  font-size: .78rem;
  font-weight: 500;
}

.btn.outline {
  background: transparent;
  color: var(--vp-c-text-1);
  border-color: var(--line);
}
.btn.outline:hover { border-color: var(--brand); color: var(--brand); }

/* scroll hint */
.scroll-hint {
  position: absolute;
  left: 50%;
  bottom: 48px;
  transform: translateX(-50%);
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12px;
  font-size: .68rem;
  font-weight: 600;
  letter-spacing: .25em;
  color: var(--text-dim);
}
.scroll-line {
  width: 1px;
  height: 40px;
  background: linear-gradient(to bottom, transparent, var(--text-dim), transparent);
  animation: scrollLine 2s ease-in-out infinite;
}
@keyframes scrollLine {
  0%,100% { transform: scaleY(1); opacity: .4; }
  50%     { transform: scaleY(1.3); opacity: 1; }
}

/* ───────────── marquee ───────────── */
.marquee {
  position: relative;
  z-index: 1;
  padding: 24px 0;
  border-top: 1px solid var(--line);
  border-bottom: 1px solid var(--line);
  overflow: hidden;
  background: var(--vp-c-bg-soft);
  mask-image: linear-gradient(90deg, transparent, #000 10%, #000 90%, transparent);
  -webkit-mask-image: linear-gradient(90deg, transparent, #000 10%, #000 90%, transparent);
}
.mq-track {
  display: flex;
  gap: 56px;
  animation: marquee 60s linear infinite;
  width: max-content;
  font-family: 'JetBrains Mono', monospace;
  font-size: .82rem;
  font-weight: 500;
  color: var(--text-dim);
  letter-spacing: .03em;
}
.mq-item { display: inline-flex; align-items: center; gap: 16px; white-space: nowrap; }
.mq-bullet { color: var(--brand); }
@keyframes marquee {
  from { transform: translateX(0); }
  to   { transform: translateX(-50%); }
}

/* ───────────── generic section ───────────── */
.sec {
  position: relative;
  z-index: 1;
  padding: 120px 0;
}
.sec-head { text-align: center; margin-bottom: 56px; max-width: 720px; margin-left: auto; margin-right: auto; }
.sec-label {
  display: inline-block;
  margin-bottom: 20px;
  font-family: 'JetBrains Mono', monospace;
  font-size: .8rem;
  font-weight: 500;
  letter-spacing: .08em;
  color: var(--brand);
}
.sec-title {
  font-size: clamp(1.8rem, 4vw, 3rem);
  font-weight: 700;
  line-height: 1.1;
  letter-spacing: -0.03em;
  margin: 0 0 16px;
}
.sec-desc { font-size: 1.02rem; color: var(--text-dim); line-height: 1.65; margin: 0; }

/* ───────────── tools grid ───────────── */
.tools-grid {
  display: grid;
  grid-template-columns: repeat(5, 1fr);
  gap: 14px;
  perspective: 1000px;
}
.tool {
  position: relative;
  aspect-ratio: 1.3;
  transform-style: preserve-3d;
  transform: perspective(600px) rotateX(var(--rx, 0deg)) rotateY(var(--ry, 0deg));
  transition: transform .2s cubic-bezier(.2,.7,.1,1);
  cursor: pointer;
}
.tool-inner {
  position: absolute;
  inset: 0;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 10px;
  padding: 18px 10px;
  background: var(--vp-c-bg-soft);
  border: 1px solid var(--line);
  border-radius: 16px;
  overflow: hidden;
  transition: border-color .3s, background .3s;
}
.tool:hover .tool-inner {
  border-color: rgba(96,165,250,.35);
}
.tool-glow {
  position: absolute;
  inset: 0;
  opacity: 0;
  transition: opacity .3s;
  background: radial-gradient(280px circle at var(--mx) var(--my), rgba(96,165,250,.15), transparent 70%);
  pointer-events: none;
}
.tool:hover .tool-glow { opacity: 1; }
.tool-svg {
  width: 30px; height: 30px;
  display: flex; align-items: center; justify-content: center;
  transition: transform .35s cubic-bezier(.2,.7,.1,1);
}
.tool:hover .tool-svg { transform: scale(1.12) rotate(-4deg); }
.tool-svg :deep(svg) { width: 100%; height: 100%; }
.tool-name {
  font-size: .74rem;
  color: var(--text-dim);
  font-weight: 500;
  letter-spacing: .01em;
}

/* ───────────── features ───────────── */
.feat-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 16px;
}
.feat {
  position: relative;
  padding: 1px;
  border-radius: 20px;
  overflow: hidden;
  transition: transform .4s cubic-bezier(.2,.7,.1,1);
  background: var(--line);
}
.feat:hover { transform: translateY(-4px); }
.feat-border {
  position: absolute;
  inset: 0;
  border-radius: 20px;
  opacity: 0;
  transition: opacity .35s;
  background: conic-gradient(from 0deg at var(--mx, 50%) var(--my, 50%), transparent 0deg, var(--brand) 90deg, var(--brand-2) 180deg, var(--brand-3) 270deg, transparent 360deg);
  pointer-events: none;
}
.feat:hover .feat-border { opacity: .8; }
.feat-spot {
  position: absolute;
  inset: 1px;
  border-radius: 19px;
  background: radial-gradient(400px circle at var(--mx) var(--my), rgba(96,165,250,.1), transparent 40%);
  opacity: 0;
  transition: opacity .3s;
  pointer-events: none;
}
.feat:hover .feat-spot { opacity: 1; }
.feat-body {
  position: relative;
  height: 100%;
  padding: 32px 28px;
  background: var(--vp-c-bg);
  border-radius: 19px;
}
.feat-icon {
  width: 40px; height: 40px;
  display: flex; align-items: center; justify-content: center;
  margin-bottom: 20px;
  border-radius: 10px;
  background: linear-gradient(135deg, rgba(96,165,250,.12), rgba(167,139,250,.08));
  color: var(--brand);
}
.feat-title {
  font-size: 1.08rem;
  font-weight: 600;
  margin: 0 0 10px;
  letter-spacing: -0.01em;
}
.feat-desc {
  font-size: .9rem;
  line-height: 1.65;
  color: var(--text-dim);
  margin: 0;
}
.feat-arrow {
  position: absolute;
  top: 28px; right: 24px;
  width: 32px; height: 32px;
  display: flex; align-items: center; justify-content: center;
  border-radius: 50%;
  color: var(--text-dim);
  opacity: 0;
  transform: translate(-6px, 6px) rotate(-45deg);
  transition: opacity .3s, transform .3s;
}
.feat:hover .feat-arrow {
  opacity: 1;
  transform: translate(0,0) rotate(-45deg);
  color: var(--brand);
}

/* ───────────── terminal ───────────── */
.term-wrap {
  max-width: 760px;
  margin: 0 auto;
  position: relative;
}
.term-glow {
  position: absolute;
  inset: -60px;
  background: radial-gradient(ellipse at center, rgba(96,165,250,.18), rgba(167,139,250,.08) 40%, transparent 70%);
  filter: blur(40px);
  pointer-events: none;
  z-index: 0;
}
.term {
  position: relative;
  z-index: 1;
  background: #06080f;
  border: 1px solid #1a1d2b;
  border-radius: 16px;
  overflow: hidden;
  box-shadow:
    0 0 0 1px rgba(255,255,255,.03),
    0 30px 80px -20px rgba(0,0,0,.6),
    0 0 100px -30px rgba(96,165,250,.15);
  font-family: 'JetBrains Mono', monospace;
}
.term-bar {
  display: flex;
  align-items: center;
  padding: 14px 18px;
  background: #0b0e18;
  border-bottom: 1px solid #1a1d2b;
}
.term-dots { display: flex; gap: 7px; }
.term-dots span {
  width: 11px; height: 11px;
  border-radius: 50%;
}
.term-dots span:nth-child(1) { background: #ff5f57; }
.term-dots span:nth-child(2) { background: #ffbd2e; }
.term-dots span:nth-child(3) { background: #28c840; }
.term-title {
  flex: 1;
  text-align: center;
  font-size: .72rem;
  color: #4a4e5e;
  font-family: var(--vp-font-family-base);
  letter-spacing: .02em;
}
.term-spacer { width: 52px; }

.term-body {
  padding: 24px 28px;
  min-height: 280px;
  color: #c9d1d9;
  font-size: .84rem;
  line-height: 1.7;
}
.tl {
  display: flex;
  align-items: center;
  gap: 10px;
  animation: tlIn .3s ease-out;
}
@keyframes tlIn {
  from { opacity: 0; transform: translateY(6px); }
  to   { opacity: 1; transform: none; }
}
.tl-prompt { color: #a78bfa; font-weight: 600; white-space: nowrap; }
.tl.dim .tl-text { color: #4a4e5e; }
.tl.txt .tl-text { color: #c9d1d9; margin-top: 6px; }
.tl.on .tl-text  { color: #e2e8f0; }
.tl.on .tl-text::first-letter { color: #60a5fa; }
.tl.off .tl-text { color: #4a4e5e; }
.caret {
  display: inline-block;
  width: 8px;
  color: #60a5fa;
  animation: blink 1s step-end infinite;
}
@keyframes blink { 50% { opacity: 0; } }

.tl-bar {
  margin-top: 20px;
  display: flex;
  align-items: center;
  gap: 14px;
  animation: tlIn .4s ease-out .2s both;
}
.tl-bar-track {
  flex: 1;
  height: 4px;
  background: #141725;
  border-radius: 3px;
  overflow: hidden;
}
.tl-bar-fill {
  width: 0;
  height: 100%;
  background: linear-gradient(90deg, #60a5fa, #a78bfa, #f472b6);
  border-radius: 3px;
  animation: fill 3s cubic-bezier(.3,.7,.3,1) forwards;
}
@keyframes fill {
  from { width: 0; }
  to   { width: 68%; }
}
.tl-bar-text {
  font-size: .72rem;
  color: #4a4e5e;
  white-space: nowrap;
  font-family: var(--vp-font-family-base);
}

/* ───────────── CTA ───────────── */
.cta-sec { padding: 60px 0 140px; }
.cta {
  position: relative;
  padding: 88px 40px;
  text-align: center;
  background: var(--vp-c-bg-soft);
  border: 1px solid var(--line);
  border-radius: 24px;
  overflow: hidden;
}
.cta-beam {
  position: absolute;
  top: 0; left: -30%;
  width: 60%;
  height: 2px;
  background: linear-gradient(90deg, transparent, var(--brand), transparent);
  animation: beamSweep 4s ease-in-out infinite;
}
@keyframes beamSweep {
  0%   { transform: translateX(0); }
  100% { transform: translateX(260%); }
}
.cta-grid {
  position: absolute;
  inset: 0;
  background-image:
    linear-gradient(var(--line) 1px, transparent 1px),
    linear-gradient(90deg, var(--line) 1px, transparent 1px);
  background-size: 40px 40px;
  mask-image: radial-gradient(ellipse 60% 80% at 50% 50%, #000, transparent);
  -webkit-mask-image: radial-gradient(ellipse 60% 80% at 50% 50%, #000, transparent);
  opacity: .6;
}
.cta-label {
  position: relative;
  font-family: 'JetBrains Mono', monospace;
  font-size: .8rem;
  color: var(--brand);
  margin: 0 0 16px;
  letter-spacing: .06em;
}
.cta-title {
  position: relative;
  font-size: clamp(1.6rem, 3vw, 2.4rem);
  font-weight: 700;
  line-height: 1.25;
  letter-spacing: -0.02em;
  margin: 0 0 36px;
}
.cta-btns {
  position: relative;
  display: flex;
  justify-content: center;
  gap: 12px;
  flex-wrap: wrap;
}

/* ───────────── responsive ───────────── */
@media (max-width: 960px) {
  .tools-grid { grid-template-columns: repeat(4, 1fr); }
  .feat-grid  { grid-template-columns: repeat(2, 1fr); }
}
@media (max-width: 640px) {
  .wrap { padding: 0 20px; }
  .hero { padding: 120px 0 60px; min-height: auto; }
  .headline { letter-spacing: -0.03em; }
  .install { font-size: .78rem; padding: 12px 6px 12px 14px; gap: 10px; }
  .i-action { padding: 5px 8px; }
  .tools-grid { grid-template-columns: repeat(3, 1fr); gap: 10px; }
  .tool { aspect-ratio: 1.15; }
  .feat-grid  { grid-template-columns: 1fr; }
  .feat-body  { padding: 28px 24px; }
  .sec { padding: 80px 0; }
  .cta { padding: 56px 24px; }
  .scroll-hint { display: none; }
}

@media (prefers-reduced-motion: reduce) {
  *, *::before, *::after {
    animation-duration: .01ms !important;
    animation-iteration-count: 1 !important;
    transition-duration: .01ms !important;
  }
  [data-reveal], [data-reveal-group] > *, .line .fill {
    opacity: 1 !important;
    transform: none !important;
  }
}
</style>
