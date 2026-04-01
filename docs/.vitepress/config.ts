import { defineConfig } from 'vitepress'

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
        text: 'v0.2.7',
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
  }
})
