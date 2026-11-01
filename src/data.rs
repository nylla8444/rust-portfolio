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
    jobs: &[Job {
        title: "Computer Science Student",
        company: "West Visayas State University",
        website: "https://wvsu.edu.ph",
    }],
    address: "Iloilo City, Philippines",
    time_zone: "Asia/Manila",
    email: "ledesmaallynralf@gmail.com",
    phone: "+63 908 661 7484",
    about: &[
        "Hi, I'm Allyn, though most people online know me as nylla. I study Computer Science at West Visayas State University in Iloilo City.",
        "I enjoy building fast, well-crafted software with Rust, and I'm looking for an internship where I can ship real features and learn from a great team.",
        "Creator of dioxus-vercel, a CLI that deploys Dioxus web apps to Vercel from Windows, macOS or Linux.",
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
    Fixed(u64),
    /// Stars across every public repo you own, forks excluded, fetched live.
    GitHubStars,
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
    // The X API is paid, so update this by hand every few weeks.
    Stat { label: "X followers", value: StatValue::Fixed(0) },
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
            StackItem { title: "SQL", href: "https://en.wikipedia.org/wiki/SQL" },
        ],
    ),
    (
        "Frameworks",
        &[
            StackItem { title: "Dioxus", href: "https://dioxuslabs.com" },
            StackItem { title: "Axum", href: "https://github.com/tokio-rs/axum" },
            StackItem { title: "Tailwind CSS", href: "https://tailwindcss.com" },
        ],
    ),
    (
        "Tools",
        &[
            StackItem { title: "Git", href: "https://git-scm.com" },
            StackItem { title: "GitHub", href: "https://github.com" },
            StackItem { title: "Vercel", href: "https://vercel.com" },
            StackItem { title: "RustRover", href: "https://www.jetbrains.com/rust/" },
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
        id: "open-source",
        company: "Open Source",
        website: "https://github.com/nylla8444",
        location: "Remote",
        location_type: "Remote",
        current: true,
        positions: &[Position {
            title: "Creator of dioxus-vercel",
            employment_type: "Project",
            start: "10.2026",
            end: None,
            description: &[
                "Built and published a Rust CLI that deploys Dioxus web apps to Vercel from Windows, macOS or Linux.",
                "Handles bundling with dx, project linking and prebuilt uploads, so Vercel never needs a Rust toolchain.",
            ],
            skills: &["Rust", "CLI", "Vercel", "Dioxus", "crates.io"],
            expanded: true,
        }],
    },
    Experience {
        id: "company-a",
        company: "Company Name",
        website: "https://example.com",
        location: "City, Country",
        location_type: "On-site",
        current: false,
        positions: &[Position {
            title: "Software Engineer Intern",
            employment_type: "Internship",
            start: "06.2026",
            end: Some("08.2026"),
            description: &["Placeholder. Describe what you built and the impact it had."],
            skills: &["Rust", "PostgreSQL", "Docker"],
            expanded: false,
        }],
    },
    Experience {
        id: "org-b",
        company: "Student Organization",
        website: "https://example.com",
        location: "City, Country",
        location_type: "On-site",
        current: false,
        positions: &[
            Position {
                title: "Tech Lead",
                employment_type: "Volunteer",
                start: "2025",
                end: Some("2026"),
                description: &["Placeholder. Led a small team building the organization's website."],
                skills: &["Leadership", "Web"],
                expanded: false,
            },
            Position {
                title: "Member",
                employment_type: "Volunteer",
                start: "2024",
                end: Some("2025"),
                description: &[],
                skills: &[],
                expanded: false,
            },
        ],
    },
    Experience {
        id: "freelance",
        company: "Freelance",
        website: "https://example.com",
        location: "Remote",
        location_type: "Remote",
        current: false,
        positions: &[Position {
            title: "Web Developer",
            employment_type: "Part-time",
            start: "2023",
            end: Some("2024"),
            description: &["Placeholder. Small websites for local clients."],
            skills: &["HTML", "CSS", "JavaScript"],
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

pub const EDUCATION: &[Education] = &[
    Education {
        id: "wvsu",
        school: "West Visayas State University",
        start: "20XX",
        end: None,
        degree: "Bachelor of Science",
        field: "Computer Science",
        description: &["Placeholder. Relevant coursework, clubs or achievements."],
        skills: &["Algorithms", "Data Structures", "Databases"],
        expanded: true,
    },
    Education {
        id: "high-school",
        school: "Your High School",
        start: "2019",
        end: Some("2023"),
        degree: "",
        field: "",
        description: &[],
        skills: &[],
        expanded: false,
    },
];

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
            "A CLI that deploys Dioxus web apps to Vercel. It builds with dx on your machine and uploads a prebuilt static site, so Vercel never needs Rust.",
            "Published on crates.io with releases, docs and a live demo.",
        ],
        skills: &["Rust", "CLI", "Vercel", "Dioxus", "WebAssembly"],
        expanded: true,
    },
    Project {
        title: "This portfolio",
        start: "11.2026",
        end: None,
        link: "https://github.com/nylla8444",
        description: &["Written in Rust with Dioxus and Tailwind CSS, deployed with dioxus-vercel."],
        skills: &["Rust", "Dioxus", "Tailwind CSS"],
        expanded: false,
    },
    Project {
        title: "Project Three",
        start: "2026",
        end: Some("2026"),
        link: "https://example.com",
        description: &["Placeholder project description."],
        skills: &["Placeholder"],
        expanded: false,
    },
    Project {
        title: "Project Four",
        start: "2025",
        end: Some("2026"),
        link: "https://example.com",
        description: &["Placeholder project description."],
        skills: &["Placeholder"],
        expanded: false,
    },
    Project {
        title: "Project Five",
        start: "2025",
        end: Some("2025"),
        link: "https://example.com",
        description: &["Placeholder project description."],
        skills: &["Placeholder"],
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
        date: &'static str,
        url: &'static str,
    },
}

pub const RECOGNITION: &[Recognition] = &[
    Recognition::Credential {
        kind: "Certificate",
        title: "Certificate Title",
        issuer: "Issuer",
        date: "10.2026",
        url: "#",
    },
    Recognition::Award {
        title: "Hackathon Name",
        prize: "First Prize",
        date: "09.2026",
        grade: "Year 3",
        description: &["Placeholder. What you built and why it won."],
    },
    Recognition::Credential {
        kind: "Certificate",
        title: "Another Certificate",
        issuer: "Issuer",
        date: "05.2026",
        url: "#",
    },
    Recognition::Credential {
        kind: "Certificate",
        title: "Placeholder Certificate",
        issuer: "Issuer",
        date: "01.2026",
        url: "#",
    },
    Recognition::Credential {
        kind: "Certificate",
        title: "Older Certificate",
        issuer: "Issuer",
        date: "08.2025",
        url: "#",
    },
];

/// Credits shown in the footer.
pub const CREDITS: &[(&str, &str)] = &[
    ("Design: chanhdai.com by Chánh Đại", "https://github.com/ncdai/chanhdai.com"),
    ("Cat idea: ramx.in by Ramkrishna Swarnkar", "https://ramx.in"),
    ("Cat sprite & logic: oneko.js by adryd", "https://github.com/adryd325/oneko.js"),
    ("Mono font: Paper Mono", "https://github.com/paper-design/paper-mono"),
];
