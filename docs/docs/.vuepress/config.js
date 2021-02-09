/**
 * Config reference can be found at https://vuepress.vuejs.org/config/#basic-config
 */
module.exports = {
  // The title for the site. This will be the prefix for all page titles, and
  // displayed in the navbar in the default theme.
  title: "Godot Rust CLI",

  // Description for the site. This will render at a `<meta>` tag in the page
  // HTML.
  description: "Godot Rust CLI Documentation",

  themeConfig: {
    sidebarDepth: 2,
    // Adds links to the navbar.
    nav: [
      { text: 'Guide', link: '/guide/' },
    ],

    // Defines custom sidebars for pages.
    sidebar: {
      '/guide/': [
        {
          title: 'Guide',
          collapsable: false,
          children: [
            '',
            'getting-started',
            'api',
            'compatibility',
          ]
        },
      ],
    },

    // Shows the UNIX timestamp, in milliseconds, of each file's last git commit.
    lastUpdated: 'Last Updated',

    // The link to the GitHub repo.
    repo: 'robertcorponoi/godot-rust-cli',

    // Specify the directory of the docs.
    docsDir: 'docs'
  }
};