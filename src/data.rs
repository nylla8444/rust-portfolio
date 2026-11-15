//! Everything the page shows lives here. Swap the placeholders for your own
//! details; the components read from these constants and need no changes.

#[derive(PartialEq)]
pub struct User {
    pub display_name: &'static str,
    /// The name you go by. It's drawn as the 3D mark, the header logo and the
    /// big footer wordmark (letters A–Z only).
    pub alias: &'static str,
    pub taglines: &'static [&'static str],
    pub jobs: &'static [Job],
    pub address: &'static str,
    /// IANA time zone, e.g. "Asia/Manila" or "Europe/Berlin".
    pub time_zone: &'static str,
    pub email: &'static str,
    pub phone: &'static str,
    pub about: &'static [&'static str],
    pub github_username: &'static str,
    pub source_repo: &'static str,
}

#[derive(PartialEq)]
pub struct Job {
    pub title: &'static str,
    pub company: &'static str,
    pub website: &'static str,
}

pub const USER: User = User {
    display_name: "Allyn Ralf Ledesma",
    alias: "nylla",
    taglines: &[
        "aka nylla. Creating with code, small details matter.",
        "Computer Science student at WVSU.",
        "Building Rust tools that make deploys boring.",
    ],
    jobs: &[
        Job {
            title: "Software Developer Intern",
            company: "RoboCode Technologies",
            website: "https://robocodeclub.com",
        },
        Job {
            title: "Computer Science Student",
            company: "West Visayas State University",
            website: "https://wvsu.edu.ph",
        },
    ],
    address: "Iloilo City, Philippines",
    time_zone: "Asia/Manila",
    email: "ledesmaallynralf@gmail.com",
    phone: "+63 908 661 7484",
    about: &[
        "Hi, I'm Allyn, though most people online know me as nylla. I'm a fourth-year Computer Science student at West Visayas State University in Iloilo City.",
        "I intern at RoboCode Technologies, where I build ERP modules with Next.js and Directus.",
        "Outside work I build Rust tools: dioxus-vercel deploys Dioxus web apps to Vercel, and viewtally is the privacy-friendly counter behind this site's page views.",
    ],
    github_username: "nylla8444",
    source_repo: "nylla8444/rust-portfolio",
};

/// Placeholder until you register a domain (e.g. the free .me from the GitHub Student Pack).
pub const SITE_TITLE: &str = "nylla.me";
pub const SITE_SUBTITLE: &str = "Personal portfolio, built with Rust and Dioxus";

#[derive(PartialEq)]
pub struct NavItem {
    pub title: &'static str,
    pub href: &'static str,
}

pub const MAIN_NAV: &[NavItem] = &[
    // Hidden along with their sections until there's work to list.
    // NavItem { title: "Components", href: "#components" },
    // NavItem { title: "Blocks", href: "#blocks" },
    NavItem { title: "Craft", href: "#projects" },
    NavItem { title: "Blog", href: "#blog" },
    NavItem { title: "Experience", href: "#experience" },
];

#[derive(Clone, Copy, PartialEq)]
pub enum SocialName {
    X,
    GitHub,
    LinkedIn,
}

#[derive(PartialEq)]
pub struct SocialLink {
    pub name: SocialName,
    pub title: &'static str,
    pub handle: &'static str,
    pub href: &'static str,
}

pub const SOCIAL_LINKS: &[SocialLink] = &[
    SocialLink {
        name: SocialName::X,
        title: "X (formerly Twitter)",
        handle: "@nylla8444",
        href: "https://x.com/nylla8444",
    },
    SocialLink {
        name: SocialName::GitHub,
        title: "GitHub",
        handle: "nylla8444",
        href: "https://github.com/nylla8444",
    },
    SocialLink {
        name: SocialName::LinkedIn,
        title: "LinkedIn",
        handle: "ledesmaar",
        href: "https://www.linkedin.com/in/ledesmaar/",
    },
];

#[derive(PartialEq)]
pub struct Stat {
    pub label: &'static str,
    pub value: StatValue,
}

#[derive(Clone, Copy, PartialEq)]
pub enum StatValue {
    /// Typed in by hand.
    #[allow(dead_code)]
    Fixed(u64),
    /// Stars across every public repo you own, forks excluded, fetched live.
    GitHubStars,
    /// Your GitHub follower count, fetched live.
    GitHubFollowers,
    /// All-time downloads across every crate you own on crates.io, fetched live.
    CratesDownloads,
    /// All-time views of this site, from the counter in `PAGE_VIEWS`.
    PageViewsAllTime,
}

/// Your viewtally page-view counter (https://github.com/nylla8444/viewtally-rust):
/// the Worker's URL and the site id this page is counted under. The Worker only
/// counts views from the origins it lists for that site, so `dx serve` on
/// localhost can read the total without adding to it.
pub struct PageViews {
    pub endpoint: &'static str,
    pub site: &'static str,
}

pub const PAGE_VIEWS: PageViews = PageViews {
    endpoint: "https://viewtally.nylla.workers.dev",
    site: "portfolio",
};

pub const NUMBERS: &[Stat] = &[
    Stat { label: "GitHub stars", value: StatValue::GitHubStars },
    // X followers would need X's paid API; use `StatValue::Fixed(n)` to type it in by hand.
    Stat { label: "GitHub followers", value: StatValue::GitHubFollowers },
    Stat { label: "crates.io downloads", value: StatValue::CratesDownloads },
    Stat { label: "Page views", value: StatValue::PageViewsAllTime },
];

#[derive(PartialEq)]
pub struct Testimonial {
    pub quote: &'static str,
    pub author_name: &'static str,
    pub author_tagline: &'static str,
    pub url: &'static str,
    pub featured: bool,
}

const PLACEHOLDER_QUOTE: &str = "Placeholder testimonial. Replace it with a real quote from someone you've worked with.";

pub const TESTIMONIALS_1: &[Testimonial] = &[
    Testimonial { quote: PLACEHOLDER_QUOTE, author_name: "Person One", author_tagline: "Role at Company", url: "#", featured: true },
    Testimonial { quote: "Short placeholder quote.", author_name: "Person Three", author_tagline: "Role at Company", url: "#", featured: false },
    Testimonial { quote: PLACEHOLDER_QUOTE, author_name: "Person Four", author_tagline: "Role at Company", url: "#", featured: false },
    Testimonial { quote: "Another placeholder.", author_name: "Person Five", author_tagline: "Role at Company", url: "#", featured: false },
    Testimonial { quote: PLACEHOLDER_QUOTE, author_name: "Person Six", author_tagline: "Role at Company", url: "#", featured: false },
];

pub const TESTIMONIALS_2: &[Testimonial] = &[
    Testimonial { quote: "Placeholder testimonial.", author_name: "Person Two", author_tagline: "Role at Company", url: "#", featured: true },
    Testimonial { quote: "Super clean placeholder.", author_name: "Person Seven", author_tagline: "Role at Company", url: "#", featured: false },
    Testimonial { quote: PLACEHOLDER_QUOTE, author_name: "Person Eight", author_tagline: "Role at Company", url: "#", featured: false },
    Testimonial { quote: "Fun placeholder site!", author_name: "Person Nine", author_tagline: "Role at Company", url: "#", featured: false },
    Testimonial { quote: PLACEHOLDER_QUOTE, author_name: "Person Ten", author_tagline: "Role at Company", url: "#", featured: false },
];

#[derive(PartialEq)]
pub struct ComponentDoc {
    pub title: &'static str,
    pub new: bool,
}

/// Not shown yet: the Components section is commented out in main.rs.
#[allow(dead_code)]
pub const COMPONENTS: &[ComponentDoc] = &[
    ComponentDoc { title: "Isometric Mark", new: true },
    ComponentDoc { title: "Oneko Cat", new: true },
    ComponentDoc { title: "Contribution Graph", new: false },
    ComponentDoc { title: "Text Swap", new: false },
    ComponentDoc { title: "Local Time", new: false },
    ComponentDoc { title: "Theme Toggle", new: false },
    ComponentDoc { title: "Logos Carousel", new: false },
    ComponentDoc { title: "Dimension Lines", new: false },
    ComponentDoc { title: "Command Menu", new: false },
];

#[derive(PartialEq)]
pub struct Card {
    pub title: &'static str,
    pub subtitle: &'static str,
}

/// Not shown yet: the Blocks section is commented out in main.rs.
#[allow(dead_code)]
pub const BLOCKS: &[Card] = &[
    Card { title: "Profile Header 01", subtitle: "Avatar, name and rotating taglines." },
    Card { title: "Overview 01", subtitle: "Job, location, local time and contacts." },
    Card { title: "Experience 01", subtitle: "Collapsible work history with tags." },
    Card { title: "Footer 01", subtitle: "Footer laid out like a technical drawing." },
];

pub const POSTS: &[Card] = &[
    Card { title: "Shipping dioxus-vercel 0.1", subtitle: "30 Oct 2026" },
    Card { title: "Why I built my portfolio in Rust", subtitle: "1 Nov 2026" },
];

#[derive(PartialEq)]
pub struct StackItem {
    pub title: &'static str,
    pub href: &'static str,
}

pub const TECH_STACK: &[(&str, &[StackItem])] = &[
    (
        "Languages",
        &[
            StackItem { title: "Rust", href: "https://www.rust-lang.org" },
            StackItem { title: "TypeScript", href: "https://www.typescriptlang.org" },
            StackItem { title: "JavaScript", href: "https://developer.mozilla.org/en-US/docs/Web/JavaScript" },
            StackItem { title: "Python", href: "https://www.python.org" },
            StackItem { title: "Dart", href: "https://dart.dev" },
            StackItem { title: "SQL", href: "https://en.wikipedia.org/wiki/SQL" },
        ],
    ),
    (
        "Frameworks",
        &[
            StackItem { title: "Dioxus", href: "https://dioxuslabs.com" },
            StackItem { title: "React", href: "https://react.dev" },
            StackItem { title: "Next.js", href: "https://nextjs.org" },
            StackItem { title: "Node.js", href: "https://nodejs.org" },
            StackItem { title: "Express", href: "https://expressjs.com" },
            StackItem { title: "Flutter", href: "https://flutter.dev" },
            StackItem { title: "Tailwind CSS", href: "https://tailwindcss.com" },
        ],
    ),
    (
        "Backend & data",
        &[
            StackItem { title: "Directus", href: "https://directus.io" },
            StackItem { title: "Strapi", href: "https://strapi.io" },
            StackItem { title: "Redis", href: "https://redis.io" },
            StackItem { title: "MongoDB", href: "https://www.mongodb.com" },
            StackItem { title: "Cloudflare D1", href: "https://developers.cloudflare.com/d1/" },
        ],
    ),
    (
        "Tools & platforms",
        &[
            StackItem { title: "Git", href: "https://git-scm.com" },
            StackItem { title: "GitHub", href: "https://github.com" },
            StackItem { title: "Docker", href: "https://www.docker.com" },
            StackItem { title: "Vercel", href: "https://vercel.com" },
            StackItem { title: "Cloudflare Workers", href: "https://workers.cloudflare.com" },
        ],
    ),
];

#[derive(PartialEq)]
pub struct Experience {
    pub id: &'static str,
    pub company: &'static str,
    pub website: &'static str,
    pub location: &'static str,
    pub location_type: &'static str,
    pub current: bool,
    pub positions: &'static [Position],
}

#[derive(PartialEq)]
pub struct Position {
    pub title: &'static str,
    pub employment_type: &'static str,
    pub start: &'static str,
    pub end: Option<&'static str>,
    pub description: &'static [&'static str],
    pub skills: &'static [&'static str],
    pub expanded: bool,
}

pub const EXPERIENCES: &[Experience] = &[
    Experience {
        id: "robocode",
        company: "RoboCode Technologies",
        website: "https://robocodeclub.com",
        location: "Philippines",
        location_type: "Remote",
        current: true,
        positions: &[Position {
            title: "Software Developer Intern",
            employment_type: "Internship",
            start: "12.2025",
            end: None,
            description: &[
                "Building an in-house ERP web app with Next.js and Directus, across finance, inventory, kitchen and banquet operations.",
                "Wrote a reusable Next.js + Directus starter with email/password and Google sign-in, protected routes and permission checks.",
                "Built the game catalog service for the company's learning platform: a Node.js API that bundles browser games into single HTML files, gates them with JWTs and caches them in Redis, refreshed by Strapi webhooks.",
            ],
            skills: &["Next.js", "TypeScript", "Directus", "Node.js", "Redis", "Strapi", "Docker"],
            expanded: true,
        }],
    },
    Experience {
        id: "open-source",
        company: "Open Source",
        website: "https://github.com/nylla8444",
        location: "Worldwide",
        location_type: "Remote",
        current: true,
        positions: &[Position {
            title: "Creator of dioxus-vercel and viewtally",
            employment_type: "Project",
            start: "10.2026",
            end: None,
            description: &[
                "dioxus-vercel: a Rust CLI, published on crates.io, that deploys Dioxus web apps to Vercel, either as a prebuilt upload or as a Git-connected build on every push.",
                "viewtally: a privacy-friendly page-view counter in Rust on Cloudflare Workers and D1, also on crates.io. It counts the views on this site.",
                "My Dioxus issue about dx failing on Vercel's build image (DioxusLabs/dioxus#6050) led to an upstream fix that brought back static musl builds.",
            ],
            skills: &["Rust", "Dioxus", "WebAssembly", "Vercel", "Cloudflare Workers", "crates.io"],
            expanded: false,
        }],
    },
];

#[derive(PartialEq)]
pub struct Education {
    pub id: &'static str,
    pub school: &'static str,
    pub start: &'static str,
    pub end: Option<&'static str>,
    pub degree: &'static str,
    pub field: &'static str,
    pub description: &'static [&'static str],
    pub skills: &'static [&'static str],
    pub expanded: bool,
}

pub const EDUCATION: &[Education] = &[Education {
    id: "wvsu",
    school: "West Visayas State University",
    start: "2023",
    end: None,
    degree: "Bachelor of Science",
    field: "Computer Science",
    description: &[
        "Coursework in parallel and distributed computing, intelligent systems, natural language processing, computer vision, automata theory and information security.",
        "Won Best Game Design and 2nd Runner Up at the Tech4All Summit 2025 GameON competition.",
    ],
    skills: &["Algorithms", "Distributed Systems", "Automata Theory", "Machine Learning", "Software Engineering"],
    expanded: true,
}];

#[derive(PartialEq)]
pub struct Project {
    pub title: &'static str,
    pub start: &'static str,
    pub end: Option<&'static str>,
    pub link: &'static str,
    pub description: &'static [&'static str],
    pub skills: &'static [&'static str],
    pub expanded: bool,
}

pub const PROJECTS: &[Project] = &[
    Project {
        title: "dioxus-vercel",
        start: "10.2026",
        end: None,
        link: "https://crates.io/crates/dioxus-vercel",
        description: &[
            "A CLI that deploys Dioxus web apps to Vercel. Build on your machine and upload a prebuilt static site, or run `init` once and let Vercel build on every push.",
            "Published on crates.io with releases, docs, measured build times and live demos.",
        ],
        skills: &["Rust", "CLI", "Vercel", "Dioxus", "WebAssembly"],
        expanded: true,
    },
    Project {
        title: "viewtally",
        start: "10.2026",
        end: None,
        link: "https://github.com/nylla8444/viewtally-rust",
        description: &[
            "A privacy-friendly page-view counter for personal websites, written in Rust for Cloudflare Workers and D1.",
            "Counts each visitor once a day without cookies or stored IP addresses, filters bots, serves several sites from one deployment and stays within the free plan.",
        ],
        skills: &["Rust", "Cloudflare Workers", "D1", "WebAssembly"],
        expanded: false,
    },
    Project {
        title: "This portfolio",
        start: "11.2026",
        end: None,
        link: "https://github.com/nylla8444/rust-portfolio",
        description: &[
            "Written in Rust with Dioxus and Tailwind CSS, deployed with dioxus-vercel. It includes a Rust port of the oneko cursor cat and live GitHub, crates.io and viewtally numbers.",
        ],
        skills: &["Rust", "Dioxus", "Tailwind CSS"],
        expanded: false,
    },
    Project {
        title: "pyDFS",
        start: "04.2026",
        end: Some("05.2026"),
        link: "https://github.com/nylla8444/pyDFS",
        description: &[
            "A simplified distributed file system with primary-backup replication, automatic failover and crash recovery, built on Python sockets.",
        ],
        skills: &["Python", "Sockets", "Distributed Systems"],
        expanded: false,
    },
    Project {
        title: "DFA Visualizer",
        start: "10.2025",
        end: Some("11.2025"),
        link: "https://visualize-dfa.vercel.app",
        description: &[
            "An interactive web app that shows step by step how a deterministic finite automaton reads its input. Final project for Automata Theory.",
        ],
        skills: &["TypeScript", "React", "TanStack Router", "Tailwind CSS"],
        expanded: false,
    },
    Project {
        title: "sAIveon",
        start: "10.2025",
        end: Some("11.2025"),
        link: "https://github.com/nylla8444/sAIveon",
        description: &[
            "An offline-first personal finance app with budgets, scheduled payments and spending analytics, plus an AI finance advisor powered by Gemini. Final project for Application Development.",
        ],
        skills: &["Flutter", "Dart", "Gemini API"],
        expanded: false,
    },
    Project {
        title: "Cloaxa",
        start: "07.2025",
        end: Some("07.2025"),
        link: "https://github.com/nylla8444/Cloaxa",
        description: &[
            "A Chromium extension that routes browsing through Tor, blocks trackers and fingerprinting, and has a kill switch so the real IP never leaks.",
        ],
        skills: &["JavaScript", "Manifest V3", "Tor", "Privacy"],
        expanded: false,
    },
    Project {
        title: "StraySpot",
        start: "04.2025",
        end: Some("05.2025"),
        link: "https://github.com/nylla8444/strayspot",
        description: &[
            "A full-stack platform that connects animal shelters with adopters: pet profiles and search, a digital adoption workflow and accounts for each role. Final project for Software Engineering.",
        ],
        skills: &["Next.js", "MongoDB", "Tailwind CSS"],
        expanded: false,
    },
    Project {
        title: "NIMJA",
        start: "12.2024",
        end: Some("06.2025"),
        link: "https://github.com/nylla8444/NIMJA",
        description: &["The game of Nim with an AI opponent and a strategy helper that suggests the best move, using Nim-sum analysis and minimax with alpha-beta pruning."],
        skills: &["Python", "Minimax", "Game AI"],
        expanded: false,
    },
    Project {
        title: "DreamCatcher",
        start: "11.2024",
        end: Some("11.2024"),
        link: "https://github.com/nylla8444/DreamCatcher",
        description: &[
            "An Arduino baby monitor with a Node.js dashboard: live temperature and motion readings, history charts and Telegram alerts for caregivers.",
        ],
        skills: &["Arduino", "Node.js", "Express", "Socket.IO"],
        expanded: false,
    },
];

#[derive(PartialEq)]
pub enum Recognition {
    Award {
        title: &'static str,
        prize: &'static str,
        date: &'static str,
        grade: &'static str,
        description: &'static [&'static str],
    },
    // Kept for certificates you add later.
    #[allow(dead_code)]
    Credential {
        kind: &'static str,
        title: &'static str,
        issuer: &'static str,
        date: &'static str,
        url: &'static str,
    },
}

pub const RECOGNITION: &[Recognition] = &[
    Recognition::Award {
        title: "GameON: Game Development Competition",
        prize: "Best Game Design",
        date: "09.2025",
        grade: "Year 3",
        description: &["Tech4All Summit 2025, part of the Western Visayas Startup Fest, at the Iloilo Convention Center."],
    },
    Recognition::Award {
        title: "GameON: Game Development Competition",
        prize: "2nd Runner Up",
        date: "09.2025",
        grade: "Year 3",
        description: &["Tech4All Summit 2025, part of the Western Visayas Startup Fest, at the Iloilo Convention Center."],
    },
    Recognition::Award {
        title: "2025 National AI Hackathon",
        prize: "Top 10",
        date: "08.2025",
        grade: "Year 3",
        description: &["AI Fest 2025, held August 11–13 at Richmonde Hotel, Iloilo City."],
    },
];

/// Credits shown in the footer.
pub const CREDITS: &[(&str, &str)] = &[
    ("Design: chanhdai.com by Chánh Đại", "https://github.com/ncdai/chanhdai.com"),
    ("Cat idea: ramx.in by Ramkrishna Swarnkar", "https://ramx.in"),
    ("Cat sprite & logic: oneko.js by adryd", "https://github.com/adryd325/oneko.js"),
    ("Mono font: Paper Mono", "https://github.com/paper-design/paper-mono"),
];
