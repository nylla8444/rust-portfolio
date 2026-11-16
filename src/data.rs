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
            title: "Software Developer",
            company: "Robo Code Technologies Inc.",
            website: "https://rcti.ph",
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
        "Hi, I'm Allyn, though most people online know me as nylla. I'm a fourth-year Computer Science student at West Visayas State University in Iloilo City, graduating in 2027.",
        "I'm a part-time software developer at Robo Code Technologies Inc., building a full-stack ERP for food and commissary businesses with Next.js and Directus.",
        "For my thesis, ECHO, I built the machine-learning pipeline and GPU inference API that screen retinal photos for hypertension risk.",
        "Outside work I build Rust tools: dioxus-vercel deploys Dioxus web apps to Vercel, and viewtally is the privacy-friendly counter behind this site's page views.",
    ],
    github_username: "nylla8444",
    source_repo: "nylla8444/rust-portfolio",
};

/// Switch to your own domain once it's registered (e.g. the free .me from the GitHub Student Pack).
pub const SITE_TITLE: &str = "nylla.vercel.app";
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
    // Hidden along with the Blog section until there are posts.
    // NavItem { title: "Blog", href: "#blog" },
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

// Not shown yet: the Testimonials section is commented out in main.rs until
// these are real quotes.
#[allow(dead_code)]
pub const TESTIMONIALS_1: &[Testimonial] = &[
    Testimonial { quote: PLACEHOLDER_QUOTE, author_name: "Person One", author_tagline: "Role at Company", url: "#", featured: true },
    Testimonial { quote: "Short placeholder quote.", author_name: "Person Three", author_tagline: "Role at Company", url: "#", featured: false },
    Testimonial { quote: PLACEHOLDER_QUOTE, author_name: "Person Four", author_tagline: "Role at Company", url: "#", featured: false },
    Testimonial { quote: "Another placeholder.", author_name: "Person Five", author_tagline: "Role at Company", url: "#", featured: false },
    Testimonial { quote: PLACEHOLDER_QUOTE, author_name: "Person Six", author_tagline: "Role at Company", url: "#", featured: false },
];

#[allow(dead_code)]
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

/// Not shown yet: the Blog section is commented out in main.rs until these posts exist.
#[allow(dead_code)]
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
            StackItem { title: "HTML", href: "https://developer.mozilla.org/en-US/docs/Web/HTML" },
            StackItem { title: "CSS", href: "https://developer.mozilla.org/en-US/docs/Web/CSS" },
        ],
    ),
    (
        "Web & mobile",
        &[
            StackItem { title: "React", href: "https://react.dev" },
            StackItem { title: "Next.js", href: "https://nextjs.org" },
            StackItem { title: "Vite", href: "https://vite.dev" },
            StackItem { title: "TanStack Query", href: "https://tanstack.com/query" },
            StackItem { title: "TanStack Router", href: "https://tanstack.com/router" },
            StackItem { title: "Tailwind CSS", href: "https://tailwindcss.com" },
            StackItem { title: "Sass", href: "https://sass-lang.com" },
            StackItem { title: "Dioxus", href: "https://dioxuslabs.com" },
            StackItem { title: "Flutter", href: "https://flutter.dev" },
            StackItem { title: "Expo", href: "https://expo.dev" },
        ],
    ),
    (
        "Backend",
        &[
            StackItem { title: "Node.js", href: "https://nodejs.org" },
            StackItem { title: "Bun", href: "https://bun.sh" },
            StackItem { title: "Express", href: "https://expressjs.com" },
            StackItem { title: "Axum", href: "https://github.com/tokio-rs/axum" },
            StackItem { title: "FastAPI", href: "https://fastapi.tiangolo.com" },
            StackItem { title: "Socket.IO", href: "https://socket.io" },
            StackItem { title: "JWT auth", href: "https://jwt.io" },
            StackItem { title: "Strapi", href: "https://strapi.io" },
            StackItem { title: "Directus", href: "https://directus.io" },
        ],
    ),
    (
        "Databases",
        &[
            StackItem { title: "PostgreSQL", href: "https://www.postgresql.org" },
            StackItem { title: "MongoDB", href: "https://www.mongodb.com" },
            StackItem { title: "MySQL", href: "https://www.mysql.com" },
            StackItem { title: "SQLite (Drift)", href: "https://drift.simonbinder.eu" },
            StackItem { title: "Redis", href: "https://redis.io" },
            StackItem { title: "Cloudflare D1", href: "https://developers.cloudflare.com/d1/" },
        ],
    ),
    (
        "Infrastructure",
        &[
            StackItem { title: "Git", href: "https://git-scm.com" },
            StackItem { title: "GitHub Actions", href: "https://github.com/features/actions" },
            StackItem { title: "Docker", href: "https://www.docker.com" },
            StackItem { title: "Vercel", href: "https://vercel.com" },
            StackItem { title: "Cloudflare Workers", href: "https://workers.cloudflare.com" },
            StackItem { title: "Modal", href: "https://modal.com" },
            StackItem { title: "crates.io", href: "https://crates.io" },
            StackItem { title: "PowerShell", href: "https://learn.microsoft.com/powershell/" },
        ],
    ),
    (
        "AI & ML",
        &[
            StackItem { title: "PyTorch", href: "https://pytorch.org" },
            StackItem { title: "scikit-learn", href: "https://scikit-learn.org" },
            StackItem { title: "OpenCV", href: "https://opencv.org" },
            StackItem { title: "U-Net segmentation", href: "https://arxiv.org/abs/1505.04597" },
            StackItem { title: "Gemini API", href: "https://ai.google.dev" },
            StackItem { title: "Jupyter", href: "https://jupyter.org" },
        ],
    ),
    (
        "Web3 & other",
        &[
            StackItem { title: "Solana", href: "https://solana.com" },
            StackItem { title: "MetaMask auth", href: "https://metamask.io" },
            StackItem { title: "Arduino", href: "https://www.arduino.cc" },
            StackItem { title: "Figma", href: "https://www.figma.com" },
            StackItem { title: "Photoshop", href: "https://www.adobe.com/products/photoshop.html" },
        ],
    ),
];

#[derive(PartialEq)]
pub struct Experience {
    pub id: &'static str,
    pub company: &'static str,
    /// Leave empty for no link.
    pub website: &'static str,
    pub location: &'static str,
    /// Shown in brackets after the location; leave empty to hide.
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
        company: "Robo Code Technologies Inc.",
        website: "https://rcti.ph",
        location: "Philippines",
        location_type: "Remote",
        current: true,
        positions: &[
            Position {
                title: "Software Developer",
                employment_type: "Part-time",
                start: "07.2026",
                end: None,
                description: &[
                    "Building a full-stack ERP for food and commissary businesses, covering inventory, production costing, procurement, finance and banquets. 220+ commits, mostly in Banquet, Finance and Inventory.",
                    "Banquet: the event lifecycle from sales calls to per-head ingredient costing, billing and equipment dispatch, plus server-generated BEO PDFs emailed to departments with audit trails.",
                    "Finance: the chart of accounts and journal entries, a fixed asset register with monthly depreciation runs and capitalization journals, barcode asset tags, maintenance workflows and imprest petty cash.",
                    "Inventory & Kitchen: stock on hand with average cost and stock value, receiving with lot split/merge, production runs with recipe hierarchies, live kitchen order boards and spoilage tracking.",
                    "Access control: hub- and outlet-scoped permissions, managed sessions and Google sign-in.",
                ],
                skills: &["Next.js", "React", "TypeScript", "Tailwind CSS", "Directus", "PostgreSQL", "Docker", "React PDF", "Nodemailer"],
                expanded: true,
            },
            Position {
                title: "Floating Support Developer",
                employment_type: "Contract",
                start: "04.2026",
                end: Some("05.2026"),
                description: &["Set up Directus in Docker and turned stakeholder interviews into user stories and data flow diagrams."],
                skills: &["Directus", "Docker"],
                expanded: false,
            },
            Position {
                title: "Software Developer Intern",
                employment_type: "Internship",
                start: "12.2025",
                end: Some("01.2026"),
                description: &[
                    "Architected a Node.js game management service on Strapi CMS with JWT auth and a Redis + disk cache (90%+ hit rate, 70% fewer API calls), serving game progress in under 20ms.",
                    "Built a game bundler that compiles HTML, CSS, JavaScript and Blockly programs into single-file bundles, with minification and CSP hash generation against XSS.",
                    "Hardened the auth middleware: Redis token management, CORS allowlists, rate limiting and timing-safe key checks.",
                ],
                skills: &["Node.js", "Express", "Strapi", "Redis", "Blockly"],
                expanded: false,
            },
        ],
    },
    Experience {
        id: "open-source",
        company: "Open Source",
        website: "https://github.com/nylla8444",
        location: "Worldwide",
        location_type: "Remote",
        current: true,
        positions: &[
            Position {
                title: "Creator of dioxus-vercel and viewtally",
                employment_type: "Project",
                start: "10.2026",
                end: None,
                description: &[
                    "dioxus-vercel: a Rust CLI on crates.io that deploys Dioxus web apps to Vercel three ways: Vercel-side builds, GitHub Actions, or a 20-second deploy from your machine.",
                    "viewtally: a privacy-friendly page-view counter in Rust on Cloudflare Workers + D1, also on crates.io. It powers this site's view count.",
                ],
                skills: &["Rust", "Dioxus", "WebAssembly", "Vercel", "Cloudflare Workers", "crates.io"],
                expanded: false,
            },
            Position {
                title: "Contributor",
                employment_type: "Open source",
                start: "09.2024",
                end: None,
                description: &[
                    "Opened 20+ issues and landed 25+ merged PRs in other people's repositories.",
                    "Dioxus (39k stars): reported that dx's Linux binaries needed glibc 2.35+, which broke builds on Vercel. Fixed upstream in PR #6052.",
                    "hackathon-starter (35k stars): opened an issue and a merged PR removing stale SendGrid setup from the docs and .env.example.",
                    "Gemini CLI (108k stars) and Zed (94k stars): reported Windows input and account bugs.",
                    "BINHI Website 2025 (SPARK Hub WVSU): 5 merged PRs for the homepage, testimonials and CTA sections, plus cleanup issues.",
                ],
                skills: &[],
                expanded: false,
            },
        ],
    },
    Experience {
        id: "theblokc",
        company: "The BLOKC",
        website: "https://theblokc.com",
        location: "Philippines",
        location_type: "Remote",
        current: false,
        positions: &[Position {
            title: "Software Engineer Intern",
            employment_type: "Internship",
            start: "07.2023",
            end: Some("08.2023"),
            description: &["Built a responsive digital business card web app for the team's contacts from design mockups."],
            skills: &["React", "Tailwind CSS"],
            expanded: false,
        }],
    },
    Experience {
        id: "cipher",
        company: "CIPHER, WVSU",
        website: "https://github.com/WVSU-CIPHER",
        location: "Iloilo City",
        location_type: "",
        current: true,
        positions: &[Position {
            title: "R&D Committee Member (Front-end Developer)",
            employment_type: "Student org",
            start: "07.2024",
            end: None,
            description: &[
                "Built the front end for the CICT Tiringbanay 2024 website (16 merged PRs, including the groups and student search pages) and the CICT and CBM Fundays 2024 websites.",
            ],
            skills: &["Next.js", "React", "MDX"],
            expanded: false,
        }],
    },
    Experience {
        id: "cyb-robotics",
        company: "Cyb Robotics Organization, WVSU",
        website: "",
        location: "Iloilo City",
        location_type: "",
        current: false,
        positions: &[Position {
            title: "Vice President for Programs",
            employment_type: "Student org",
            start: "08.2024",
            end: Some("06.2025"),
            description: &[
                "Led the programs committee: planned events, delegated tasks and kept deliverables on schedule.",
                "Headed CYBergence: From Concepts to Creations (Sept 2024), produced materials for SHErcuit 2025 and CYBOARD 2025, and gave on-site programming support.",
            ],
            skills: &["Leadership", "Event planning"],
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
    end: Some("2027"),
    degree: "Bachelor of Science",
    field: "Computer Science",
    description: &[
        "Expected to graduate in June 2027. Gold Awardee.",
        "Coursework: Software Engineering, Algorithms & Complexity, Machine Learning, Artificial Neural Networks, NLP, Automata Theory, Parallel & Distributed Computing.",
        "Thesis: ECHO, a retinal vascular screening system for hypertension risk.",
    ],
    skills: &["Software Engineering", "Algorithms", "Machine Learning", "Neural Networks", "NLP", "Distributed Systems"],
    expanded: true,
}];

#[derive(PartialEq)]
pub struct Project {
    pub title: &'static str,
    pub start: &'static str,
    pub end: Option<&'static str>,
    /// Leave empty for a project with no public link.
    pub link: &'static str,
    pub description: &'static [&'static str],
    pub skills: &'static [&'static str],
    pub expanded: bool,
}

pub const PROJECTS: &[Project] = &[
    Project {
        title: "ECHO: Retinal Vascular Screening",
        start: "2025",
        end: None,
        link: "",
        description: &[
            "My undergraduate thesis. It screens fundus photos for hypertension risk from measured retinal vessels. I built the ML pipeline and the GPU inference API.",
            "A U-Net trained from scratch segments the vessels (Dice 0.886), and vessel-width measurements land within 3–4% of independent annotations.",
            "The screening model reaches AUC 0.786 at 85% sensitivity in nested cross-validation on 712 images. Field-tested on a local eye clinic's camera, and served with FastAPI on Modal GPUs at about 7 seconds per photo.",
        ],
        skills: &["PyTorch", "scikit-learn", "OpenCV", "FastAPI", "Modal", "Next.js"],
        expanded: true,
    },
    Project {
        title: "dioxus-vercel",
        start: "10.2026",
        end: None,
        link: "https://crates.io/crates/dioxus-vercel",
        description: &[
            "A Rust CLI that deploys Dioxus web apps to Vercel from Windows, macOS or Linux, three ways: Vercel-side builds, GitHub Actions, or a 20-second deploy from your machine.",
            "Ships its own dx build to work around Vercel's older glibc. Published on crates.io.",
        ],
        skills: &["Rust", "CLI", "Vercel", "Dioxus", "GitHub Actions"],
        expanded: false,
    },
    Project {
        title: "viewtally",
        start: "10.2026",
        end: None,
        link: "https://crates.io/crates/viewtally",
        description: &[
            "A privacy-friendly page-view counter in Rust on Cloudflare Workers + D1. It's cookie-free and stores no IPs; repeat visitors are filtered with daily-rotating salted hashes.",
            "Handles about 25,000 counted views a day on the free plan, and powers this site's view count.",
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
    Credential {
        kind: &'static str,
        title: &'static str,
        issuer: &'static str,
        /// Leave empty to hide.
        date: &'static str,
        /// Leave empty for no link.
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
        title: "SIKAPTala 2025 CS & IT Skills Competition",
        prize: "Top 29",
        date: "04.2025",
        grade: "Year 2",
        description: &["Held at De La Salle University – Dasmariñas."],
    },
    Recognition::Award {
        title: "Odyssey Festival: Developers District Game Dev Hackathon",
        prize: "Champion",
        date: "08.2024",
        grade: "Year 2",
        description: &["Won with Astral Visions."],
    },
    Recognition::Award {
        title: "GDSC Solve: Solutions Challenge Visayas",
        prize: "Semi-finalist",
        date: "04.2024",
        grade: "Year 1",
        description: &[],
    },
    Recognition::Award {
        title: "DEVCON Iloilo Game Development Hackathon",
        prize: "Champion",
        date: "11.2023",
        grade: "Year 1",
        description: &[],
    },
    Recognition::Credential {
        kind: "Certificate",
        title: "Microsoft AI Skills Fest 2026",
        issuer: "Microsoft",
        date: "",
        url: "",
    },
    Recognition::Credential {
        kind: "Certificate",
        title: "GitHub Foundations",
        issuer: "GitHub",
        date: "",
        url: "",
    },
    Recognition::Credential {
        kind: "Certificate",
        title: "Google AI Essentials",
        issuer: "Google",
        date: "",
        url: "",
    },
    Recognition::Credential {
        kind: "Certificate",
        title: "AWS Educate Introduction to Generative AI",
        issuer: "Amazon Web Services",
        date: "",
        url: "",
    },
    Recognition::Credential {
        kind: "Certificate",
        title: "Problem Solving (Basic)",
        issuer: "HackerRank",
        date: "",
        url: "",
    },
    Recognition::Credential {
        kind: "Certificate",
        title: "Python (Basic)",
        issuer: "HackerRank",
        date: "",
        url: "",
    },
    Recognition::Credential {
        kind: "Badge",
        title: "Introduction to IoT",
        issuer: "Cisco Networking Academy",
        date: "10.2024",
        url: "",
    },
];

/// Credits shown in the footer.
pub const CREDITS: &[(&str, &str)] = &[
    ("Design: chanhdai.com by Chánh Đại", "https://github.com/ncdai/chanhdai.com"),
    ("Cat idea: ramx.in by Ramkrishna Swarnkar", "https://ramx.in"),
    ("Cat sprite & logic: oneko.js by adryd", "https://github.com/adryd325/oneko.js"),
    ("Key sound: carlo.tl by Carlo Taleon", "https://carlo.tl"),
    ("Mono font: Paper Mono", "https://github.com/paper-design/paper-mono"),
];
