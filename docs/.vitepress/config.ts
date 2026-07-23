import { defineConfig, type HeadConfig } from 'vitepress'

// 工具页 HowTo schema 映射
const toolHowTo: Record<string, { name: string; steps: string[] }> = {
  'tools/git': { name: '在 Windows 安装 Git', steps: ['运行 hudo install git', 'hudo 自动下载最新版 Git 并静默安装', '自动配置 PATH 环境变量和 core.autocrlf'] },
  'tools/nodejs': { name: '在 Windows 安装 Node.js', steps: ['运行 hudo install nodejs', 'hudo 下载官方预编译包并解压', '自动配置 PATH，npm 开箱即用'] },
  'tools/fnm': { name: '在 Windows 安装 fnm', steps: ['运行 hudo install fnm', 'hudo 下载 fnm 并安装最新 LTS Node.js', '自动配置 FNM_DIR 并写入 PowerShell profile'] },
  'tools/rust': { name: '在 Windows 安装 Rust', steps: ['运行 hudo install rust', 'hudo 通过 rustup 安装 Rust 工具链', '自动配置 CARGO_HOME 和 RUSTUP_HOME，支持 USTC 镜像'] },
  'tools/go': { name: '在 Windows 安装 Go', steps: ['运行 hudo install go', 'hudo 下载 Go 官方安装包', '自动配置 GOROOT、GOPATH 和 GOPROXY 国内镜像'] },
  'tools/jdk': { name: '在 Windows 安装 JDK', steps: ['运行 hudo install jdk', 'hudo 下载 Eclipse Temurin JDK', '自动配置 JAVA_HOME 和 PATH'] },
  'tools/maven': { name: '在 Windows 安装 Maven', steps: ['运行 hudo install maven', 'hudo 下载 Maven 并解压', '自动配置 MAVEN_HOME 和阿里云镜像仓库'] },
  'tools/gradle': { name: '在 Windows 安装 Gradle', steps: ['运行 hudo install gradle', 'hudo 下载 Gradle 并解压', '自动配置 GRADLE_HOME 和阿里云仓库镜像'] },
  'tools/python': { name: '在 Windows 安装 Python (uv)', steps: ['运行 hudo install uv', 'hudo 安装 uv Python 包管理器', '通过 uv 管理 Python 版本和虚拟环境'] },
  'tools/miniconda': { name: '在 Windows 安装 Miniconda', steps: ['运行 hudo install miniconda', 'hudo 下载 Miniconda 安装包', '自动静默安装并执行 conda init'] },
  'tools/mysql': { name: '在 Windows 安装 MySQL', steps: ['运行 hudo install mysql', 'hudo 下载 MySQL Community Server', '自动初始化数据库、注册 Windows 服务、配置环境变量'] },
  'tools/pgsql': { name: '在 Windows 安装 PostgreSQL', steps: ['运行 hudo install pgsql', 'hudo 下载 PostgreSQL 预编译包', '自动初始化数据目录、注册 Windows 服务'] },
  'tools/redis': { name: '在 Windows 安装 Redis', steps: ['运行 hudo install redis', 'hudo 下载 Redis Windows 预编译包', '自动注册 Windows 服务，开机自启'] },
  'tools/vscode': { name: '在 Windows 安装 VS Code', steps: ['运行 hudo install vscode', 'hudo 下载 VS Code 便携版', '自动配置 PATH 和右键菜单'] },
  'tools/pycharm': { name: '在 Windows 安装 PyCharm', steps: ['运行 hudo install pycharm', 'hudo 下载 PyCharm Community Edition', '自动解压并配置桌面快捷方式'] },
  'tools/mingw': { name: '在 Windows 安装 MinGW-w64', steps: ['运行 hudo install c', 'hudo 从 GitHub 下载 MinGW-w64 最新版', '自动配置 PATH，gcc/g++ 命令开箱即用'] },
  'tools/bun': { name: '在 Windows 安装 Bun', steps: ['运行 hudo install bun', 'hudo 下载 Bun 运行时', '自动配置 PATH 环境变量'] },
  'tools/gh': { name: '在 Windows 安装 GitHub CLI', steps: ['运行 hudo install gh', 'hudo 下载 GitHub CLI', '自动配置 PATH 并引导登录'] },
  'tools/chrome': { name: '在 Windows 安装 Google Chrome', steps: ['运行 hudo install chrome', 'hudo 下载 Chrome 企业版 MSI', '自动 UAC 提权并静默安装'] },
  'tools/claude-code': { name: '在 Windows 安装 Claude Code', steps: ['运行 hudo install claude-code', 'hudo 下载 Claude Code CLI 二进制', '自动配置 PATH 和 SHA256 校验'] },
}

export default defineConfig({
  title: 'hudo',
  description: 'Windows 开发环境一键引导工具，支持 Git/Node.js/Rust/Go/JDK/Python 等 20+ 工具自动安装配置，告别手动折腾环境变量。',
  lang: 'zh-CN',

  head: [
    ['link', { rel: 'icon', type: 'image/svg+xml', href: '/favicon.svg' }],
    // SEO meta
    ['meta', { name: 'keywords', content: 'hudo, Windows开发环境, 一键安装, 开发工具, Git安装, Node.js安装, JDK安装, Rust安装, Python环境, Go开发环境, MySQL安装, 包管理器, dev tools, Windows development' }],
    ['meta', { name: 'author', content: 'Zexa' }],
    // Open Graph
    ['meta', { property: 'og:title', content: 'hudo - Windows 开发环境一键引导工具' }],
    ['meta', { property: 'og:description', content: '一条命令安装 Git、Node.js、Rust、Go、JDK、Python 等 20+ 开发工具，自动配置环境变量，支持国内镜像加速。' }],
    ['meta', { property: 'og:type', content: 'website' }],
    ['meta', { property: 'og:url', content: 'https://hudo.zexa.cc' }],
    ['meta', { property: 'og:site_name', content: 'hudo' }],
    ['meta', { property: 'og:image', content: 'https://hudo.zexa.cc/og-image.svg' }],
    ['meta', { property: 'og:locale', content: 'zh_CN' }],
    // Twitter Card
    ['meta', { name: 'twitter:card', content: 'summary_large_image' }],
    ['meta', { name: 'twitter:title', content: 'hudo - Windows 开发环境一键引导工具' }],
    ['meta', { name: 'twitter:description', content: '一条命令安装 Git、Node.js、Rust、Go、JDK、Python 等 20+ 开发工具，告别手动折腾环境变量。' }],
    ['meta', { name: 'twitter:image', content: 'https://hudo.zexa.cc/og-image.svg' }],
    // Schema.org 结构化数据
    ['script', { type: 'application/ld+json' }, JSON.stringify({
      '@context': 'https://schema.org',
      '@type': 'SoftwareApplication',
      'name': 'hudo',
      'applicationCategory': 'DeveloperApplication',
      'operatingSystem': 'Windows',
      'description': 'Windows 开发环境一键引导工具，支持 Git/Node.js/Rust/Go/JDK/Python 等 20+ 工具自动安装配置。',
      'url': 'https://hudo.zexa.cc',
      'downloadUrl': 'https://github.com/zexadev/hudo/releases',
      'author': {
        '@type': 'Organization',
        'name': 'Zexa',
        'url': 'https://zexa.cc'
      },
      'license': 'https://opensource.org/licenses/MIT',
      'offers': {
        '@type': 'Offer',
        'price': '0',
        'priceCurrency': 'CNY'
      }
    })],
  ],

  sitemap: {
    hostname: 'https://hudo.zexa.cc',
  },

  themeConfig: {
    siteTitle: 'hudo',

    nav: [
      { text: '指南', link: '/guide/what-is-hudo' },
      { text: '工具列表', link: '/tools/' },
      { text: '博客', link: '/blog/' },
      {
        text: 'v0.3.0',
        items: [
          { text: '更新日志', link: '/changelog' },
          { text: 'GitHub', link: 'https://github.com/zexadev/hudo' },
        ]
      }
    ],

    sidebar: {
      '/guide/': [
        {
          text: '开始',
          items: [
            { text: '什么是 hudo？', link: '/guide/what-is-hudo' },
            { text: '安装', link: '/guide/install' },
            { text: '快速上手', link: '/guide/quickstart' },
          ]
        },
        {
          text: '进阶',
          items: [
            { text: '配置文件', link: '/guide/config' },
            { text: '配置档案', link: '/guide/profile' },
            { text: '自我更新', link: '/guide/update' },
          ]
        }
      ],
      '/tools/': [
        {
          text: '工具',
          items: [
            { text: '总览', link: '/tools/' },
            { text: 'Git', link: '/tools/git' },
            { text: 'GitHub CLI', link: '/tools/gh' },
            { text: 'Node.js', link: '/tools/nodejs' },
            { text: 'fnm', link: '/tools/fnm' },
            { text: 'Bun', link: '/tools/bun' },
            { text: 'Rust', link: '/tools/rust' },
            { text: 'Go', link: '/tools/go' },
            { text: 'JDK', link: '/tools/jdk' },
            { text: 'Maven', link: '/tools/maven' },
            { text: 'Gradle', link: '/tools/gradle' },
            { text: 'Python (uv)', link: '/tools/python' },
            { text: 'Miniconda', link: '/tools/miniconda' },
            { text: 'MySQL', link: '/tools/mysql' },
            { text: 'PostgreSQL', link: '/tools/pgsql' },
            { text: 'Redis', link: '/tools/redis' },
            { text: 'VS Code', link: '/tools/vscode' },
            { text: 'PyCharm', link: '/tools/pycharm' },
            { text: 'MinGW', link: '/tools/mingw' },
            { text: 'Google Chrome', link: '/tools/chrome' },
            { text: 'Claude Code', link: '/tools/claude-code' },
          ]
        }
      ],
      '/blog/': [
        {
          text: '博客',
          items: [
            { text: '全部文章', link: '/blog/' },
            { text: 'Windows Git 一键安装', link: '/blog/windows-git-install' },
            { text: 'Windows Node.js 环境搭建', link: '/blog/windows-nodejs-install' },
            { text: 'Windows JDK 一键安装', link: '/blog/windows-jdk-install' },
            { text: 'Windows MySQL 一键部署', link: '/blog/windows-mysql-install' },
            { text: 'Windows Rust 安装不踩坑', link: '/blog/windows-rust-install' },
            { text: 'Windows Redis 安装 + 开机自启', link: '/blog/windows-redis-install' },
            { text: 'uv 一键管理 Python 环境', link: '/blog/windows-python-uv-install' },
            { text: 'Miniconda 轻量 Python 环境搭建', link: '/blog/windows-miniconda-install' },
            { text: 'MinGW-w64 一键安装', link: '/blog/windows-mingw-install' },
            { text: 'Maven 安装配置一条龙', link: '/blog/windows-maven-install' },
            { text: '20 分钟配好完整开发环境', link: '/blog/windows-dev-environment-setup' },
            { text: 'PyCharm 社区版安装配置', link: '/blog/windows-pycharm-install' },
            { text: 'VS Code 便携版安装', link: '/blog/windows-vscode-install' },
            { text: 'Bun 快速上手', link: '/blog/windows-bun-install' },
            { text: '包管理器横评', link: '/blog/windows-package-manager-compare' },
            { text: 'Go 开发环境配置', link: '/blog/windows-go-install' },
            { text: 'Gradle 安装与选择', link: '/blog/windows-gradle-install' },
            { text: 'PostgreSQL 快速安装', link: '/blog/windows-postgresql-install' },
            { text: 'Claude Code AI 编程助手', link: '/blog/windows-claude-code-install' },
            { text: 'GitHub CLI 实用命令', link: '/blog/windows-gh-install' },
            { text: '一键迁移开发环境', link: '/blog/windows-dev-env-migration' },
          ]
        }
      ]
    },

    socialLinks: [
      { icon: 'github', link: 'https://github.com/zexadev/hudo' }
    ],

    footer: {
      message: '基于 MIT 协议发布',
      copyright: 'Copyright © 2025-2026 Zexa'
    },

    search: {
      provider: 'local'
    },

    editLink: {
      pattern: 'https://github.com/zexadev/hudo/edit/master/docs/:path',
      text: '在 GitHub 上编辑此页'
    }
  },

  transformHead(context) {
    const head: HeadConfig[] = []
    const { frontmatter, page } = context.pageData
    const pagePath = page.replace(/\.md$/, '').replace(/\/index$/, '')

    // 博客文章 → Article schema
    if (pagePath.startsWith('blog/') && pagePath !== 'blog') {
      const article: Record<string, unknown> = {
        '@context': 'https://schema.org',
        '@type': 'Article',
        'headline': frontmatter.title || '',
        'description': frontmatter.description || '',
        'author': { '@type': 'Organization', 'name': 'Zexa', 'url': 'https://zexa.cc' },
        'publisher': { '@type': 'Organization', 'name': 'Zexa', 'url': 'https://zexa.cc' },
        'mainEntityOfPage': `https://hudo.zexa.cc/${pagePath}`,
      }
      if (frontmatter.date) {
        article['datePublished'] = frontmatter.date
      }
      head.push(['script', { type: 'application/ld+json' }, JSON.stringify(article)])
    }

    // 工具页 → HowTo schema
    const howto = toolHowTo[pagePath]
    if (howto) {
      head.push(['script', { type: 'application/ld+json' }, JSON.stringify({
        '@context': 'https://schema.org',
        '@type': 'HowTo',
        'name': howto.name,
        'description': frontmatter.description || '',
        'step': howto.steps.map((text, i) => ({
          '@type': 'HowToStep',
          'position': i + 1,
          'text': text,
        })),
        'tool': { '@type': 'SoftwareApplication', 'name': 'hudo', 'url': 'https://hudo.zexa.cc' },
      })])
    }

    return head
  },
})
