use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread;
use serde::{Deserialize, Serialize};
use serde_json::json;

// ─── Data models ─────────────────────────────────────────────────────────────

#[derive(Serialize, Deserialize, Clone)]
struct Project { title: String, description: String, tags: Vec<String>, progress: Option<u8>, status: String }

#[derive(Serialize, Deserialize, Clone)]
struct Experience { title: String, org: String, period: String, description: String }

#[derive(Serialize, Deserialize, Clone)]
struct Skill { name: String, percentage: u8, category: String }

#[derive(Serialize, Deserialize, Clone)]
struct Education { degree: String, college: String, duration: String }

#[derive(Serialize, Deserialize, Clone)]
struct Course { name: String }

#[derive(Serialize, Deserialize, Clone)]
struct FuturePlan { title: String, period: String, description: String }

#[derive(Serialize, Deserialize, Clone)]
struct Contact { email: String, phone: String, location: String, linkedin: String, github: String, twitter: String }

#[derive(Serialize, Deserialize, Clone)]
struct Portfolio { projects: Vec<Project>, experiences: Vec<Experience>, skills: Vec<Skill>,
    education: Education, courses: Vec<Course>, plans: Vec<FuturePlan>, contact: Contact }

#[derive(Deserialize)]
struct ContactForm { name: String, email: String, subject: String, message: String }

struct State { visits: Mutex<u64>, data: Portfolio }

// ─── Portfolio data ───────────────────────────────────────────────────────────

fn data() -> Portfolio {
    Portfolio {
        projects: vec![
            Project { title: "Portfolio Website".into(), description: "First completed project — personal portfolio built with HTML/CSS/JS.".into(),
                tags: vec!["HTML".into(),"CSS".into(),"JavaScript".into(),"PostgreSQL".into()], progress: None, status: "completed".into() },
            Project { title: "MAYURI — Autonomous Drone".into(),
                description: "AI UAV that navigates battlefields, detects hidden snipers, and shares location in real time.".into(),
                tags: vec!["ROS2".into(),"Python".into(),"OpenCV".into(),"PyTorch".into()], progress: Some(29), status: "wip".into() },
            Project { title: "Text-to-Speech Synthesizer".into(),
                description: "VoiceTechForAll initiative with SPIRE Lab, IISc Bengaluru — TTS for nine major Indian languages.".into(),
                tags: vec!["Python".into(),"ML Libraries".into(),"API".into(),"Frontend".into()], progress: Some(0), status: "wip".into() },
        ],
        experiences: vec![
            Experience { title: "Marketing Lead".into(), org: "College Standards Club".into(), period: "July 2025–Present".into(),
                description: "Promoting events, raising awareness about BIS standards, and engaging students.".into() },
            Experience { title: "Google Cloud Hands-on Labs".into(), org: "Google Cloud Study Jams".into(), period: "October 2025".into(),
                description: "Completed training with hands-on experience in Compute Engine, Cloud Storage, and BigQuery.".into() },
            Experience { title: "Cloud Computing Intern".into(), org: "Eisystems Services & Technologies".into(), period: "June–Aug 2025".into(),
                description: "Built serverless apps, CI/CD pipelines, IaC projects, and image recognition application on cloud.".into() },
        ],
        skills: vec![
            Skill { name: "C/C++".into(), percentage: 73, category: "developed".into() },
            Skill { name: "HTML/CSS".into(), percentage: 76, category: "developed".into() },
            Skill { name: "Python".into(), percentage: 36, category: "developed".into() },
            Skill { name: "MySQL".into(), percentage: 29, category: "developed".into() },
            Skill { name: "Java".into(), percentage: 23, category: "developed".into() },
            Skill { name: "JavaScript".into(), percentage: 19, category: "developed".into() },
            Skill { name: "Programming & Development".into(), percentage: 40, category: "developing".into() },
            Skill { name: "AI & Machine Learning".into(), percentage: 23, category: "developing".into() },
            Skill { name: "Robotics & Control".into(), percentage: 14, category: "developing".into() },
            Skill { name: "Hardware & IoT".into(), percentage: 7, category: "developing".into() },
            Skill { name: "System Integration".into(), percentage: 4, category: "developing".into() },
        ],
        education: Education { degree: "B.Tech Computer Science & Engineering".into(),
            college: "Ramgarh Engineering College".into(), duration: "2023–2027".into() },
        courses: vec![
            Course { name: "Data Structures & Algorithms".into() },
            Course { name: "Operating Systems".into() },
            Course { name: "Computer Networks".into() },
            Course { name: "Database Management Systems".into() },
            Course { name: "Advanced Programming Practices".into() },
            Course { name: "Design & Analysis of Algorithms".into() },
        ],
        plans: vec![
            FuturePlan { title: "Complete B.Tech with honours".into(), period: "2025–2026".into(),
                description: "Focus on GPA and specialising in Programming & Development. Complete capstone project.".into() },
            FuturePlan { title: "Secure internship at top tech company".into(), period: "Summer 2025".into(),
                description: "Gain hands-on experience in production systems and enterprise security.".into() },
            FuturePlan { title: "Develop expertise in Cybersecurity".into(), period: "2025–2026".into(),
                description: "Pursue CISSP or CEH. Participate in bug bounty programs and security research.".into() },
            FuturePlan { title: "Learn Krav Maga".into(), period: "2026".into(),
                description: "Israeli self-defence system developed for the Israel Defense Forces.".into() },
            FuturePlan { title: "Contribute to open-source projects".into(), period: "Ongoing".into(),
                description: "Contribute to robotics and security open-source projects. Build reputation in developer communities.".into() },
        ],
        contact: Contact {
            email: "kpraveen206@yahoo.com".into(), phone: "+91 91024 44374".into(),
            location: "Garhwa, Jharkhand, India".into(),
            linkedin: "https://www.linkedin.com/in/praveen-kumar-2955a12a4".into(),
            github: "https://github.com/SHREE-PRAVEEN".into(),
            twitter: "https://x.com/p_rav_ee_n1082".into(),
        },
    }
}

// ─── HTML rendering helpers ───────────────────────────────────────────────────

fn esc(s: &str) -> String {
    s.replace('&',"&amp;").replace('<',"&lt;").replace('>',"&gt;").replace('"',"&quot;")
}

fn render_page(p: &Portfolio, visits: u64) -> String {
    let projects_html = {
        let completed: String = p.projects.iter().filter(|p| p.status=="completed").map(|p| {
            let tags: String = p.tags.iter().map(|t| format!(r#"<span class="tech-tag">{}</span>"#, esc(t))).collect();
            format!(r#"<div class="project-card completed"><h3>{}</h3><p>{}</p><div class="tech-tags">{}</div></div>"#,
                esc(&p.title), esc(&p.description), tags)
        }).collect();
        let wip: String = p.projects.iter().filter(|p| p.status=="wip").map(|p| {
            let tags: String = p.tags.iter().map(|t| format!(r#"<span class="tech-tag">{}</span>"#, esc(t))).collect();
            let prog = p.progress.map(|pct| format!(
                r#"<div class="progress-indicator"><div class="progress-bar"><div class="progress-fill" style="width:{}%"></div></div><span class="progress-text">{}% Complete</span></div>"#, pct, pct
            )).unwrap_or_default();
            format!(r#"<div class="project-card wip"><h3>{}</h3><p>{}</p><div class="tech-tags">{}</div>{}</div>"#,
                esc(&p.title), esc(&p.description), tags, prog)
        }).collect();
        format!(r#"<div class="two-col"><div class="col"><h2 class="sec-title">✅ Completed Projects</h2><div class="proj-list">{}</div></div>
<div class="col"><h2 class="sec-title">🔧 Work in Progress</h2><div class="proj-list">{}</div></div></div>"#, completed, wip)
    };

    let exp_html: String = p.experiences.iter().map(|e| format!(
        r#"<div class="timeline-item"><div class="tl-dot"></div><div class="tl-content">
<h3>{}</h3><div class="tl-org">{}</div><div class="tl-period">{}</div><p>{}</p></div></div>"#,
        esc(&e.title), esc(&e.org), esc(&e.period), esc(&e.description)
    )).collect();

    let dev_skills: String = p.skills.iter().filter(|s| s.category=="developed").map(|s| format!(
        r#"<div class="skill-item"><div class="skill-row"><span class="skill-name">{}</span><span class="skill-pct">{}%</span></div>
<div class="skill-bar"><div class="skill-fill" style="width:{}%"></div></div></div>"#,
        esc(&s.name), s.percentage, s.percentage
    )).collect();
    let dev2_skills: String = p.skills.iter().filter(|s| s.category=="developing").map(|s| format!(
        r#"<div class="skill-item developing"><div class="skill-row"><span class="skill-name">{}</span><span class="skill-pct">{}%</span></div>
<div class="skill-bar"><div class="skill-fill dev-fill" style="width:{}%"></div></div></div>"#,
        esc(&s.name), s.percentage, s.percentage
    )).collect();

    let courses_html: String = p.courses.iter().map(|c| format!(r#"<div class="course-item">{}</div>"#, esc(&c.name))).collect();

    let plans_html: String = p.plans.iter().map(|pl| format!(
        r#"<div class="plan-item"><div class="plan-dot"></div><div class="plan-content">
<h3>{}</h3><div class="plan-period">{}</div><p>{}</p></div></div>"#,
        esc(&pl.title), esc(&pl.period), esc(&pl.description)
    )).collect();

    let c = &p.contact;

    format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width,initial-scale=1">
<title>Portfolio | Praveen Kumar | CS Student</title>
<style>{css}</style>
</head>
<body>
<nav class="navbar">
  <div class="nav-wrap">
    <div class="nav-brand">⚙️ Praveen Kumar <span class="rust-badge">🦀 Rust</span></div>
    <div class="nav-menu" id="navMenu">
      <button class="nav-item active" data-page="1">Home</button>
      <button class="nav-item" data-page="2">Projects</button>
      <button class="nav-item" data-page="3">Experience</button>
      <button class="nav-item" data-page="4">Skills</button>
      <button class="nav-item" data-page="5">Academic</button>
      <button class="nav-item" data-page="6">Visits</button>
      <button class="nav-item" data-page="7">Future Plans</button>
      <button class="nav-item" data-page="9">Contact</button>
    </div>
    <button class="nav-toggle" id="navToggle">&#9776;</button>
  </div>
</nav>

<main class="main">

<!-- Page 1: Home -->
<section class="page active" id="page-1">
<div class="container">
  <div class="two-col home-cols">
    <div class="col center-col">
      <div class="avatar-wrap">
        <div class="avatar">PK</div>
        <p class="avatar-caption">Praveen Kumar</p>
        <div class="avatar-badge">🦀 Rust Portfolio</div>
      </div>
    </div>
    <div class="col">
      <div class="intro">
        <h1 class="name-title">Praveen Kumar</h1>
        <h2 class="prof-title">Computer Science Student</h2>
        <div class="intro-text">
          <p>Welcome to my portfolio! I'm a passionate B.Tech Computer Science student with keen interest in cybersecurity, robotics, and software development.</p>
          <p>Currently in my 3rd year at Ramgarh Engineering College, actively involved in projects ranging from IoT security to autonomous navigation.</p>
          <p>I believe in continuous learning and applying theoretical knowledge to solve real-world problems through innovative technology.</p>
        </div>
        <div class="highlights">
          <div class="hl-item"><strong>Specialisation:</strong> Embedded C, C++, Python, <span class="rust-hl">RUST</span></div>
          <div class="hl-item"><strong>Current Focus:</strong> Robotics &amp; AI</div>
          <div class="hl-item"><strong>Goal:</strong> Programming, AI &amp; ML, Hardware &amp; IoT, Robotics, Research &amp; Publishing</div>
        </div>
        <div class="powered">🦀 Served by <strong>Rust + tiny_http</strong> — zero JS frameworks</div>
      </div>
    </div>
  </div>
</div>
</section>

<!-- Page 2: Projects -->
<section class="page" id="page-2">
<div class="container">
  <h1 class="page-title">Projects Portfolio</h1>
  {projects}
</div>
</section>

<!-- Page 3: Experience -->
<section class="page" id="page-3">
<div class="container">
  <h1 class="page-title">Experience &amp; Achievements</h1>
  <div class="two-col">
    <div class="col">
      <h2 class="sec-title">💼 Professional Experience</h2>
      <div class="timeline">{exp}</div>
    </div>
    <div class="col">
      <h2 class="sec-title">🏆 Competitions &amp; Hackathons</h2>
      <div class="ach-card">
        <span class="ach-icon">🏆</span>
        <div><h3>Coming Soon</h3><p>Competition results will be listed here as they arrive.</p></div>
      </div>
    </div>
  </div>
</div>
</section>

<!-- Page 4: Skills -->
<section class="page" id="page-4">
<div class="container">
  <h1 class="page-title">Technical Skills</h1>
  <div class="two-col">
    <div class="col">
      <h2 class="sec-title">✅ Developed Skills</h2>
      <div class="skills-list">{dev_skills}</div>
    </div>
    <div class="col">
      <h2 class="sec-title">🔧 Skills in Development</h2>
      <div class="skills-list">{dev2_skills}</div>
    </div>
  </div>
</div>
</section>

<!-- Page 5: Academic -->
<section class="page" id="page-5">
<div class="container">
  <h1 class="page-title">Academic Background &amp; Objectives</h1>
  <div class="two-col">
    <div class="col">
      <h2 class="sec-title">🎓 Current Education</h2>
      <div class="edu-card">
        <h3>{degree}</h3>
        <p><strong>College:</strong> {college}</p>
        <p><strong>Duration:</strong> {duration}</p>
      </div>
      <h2 class="sec-title" style="margin-top:24px">📚 Relevant Coursework</h2>
      <div class="courses-grid">{courses}</div>
    </div>
    <div class="col">
      <h2 class="sec-title">🎯 Career Objectives</h2>
      <div class="objective-card">
        <p>I started my CS journey exploring many fields — biotechnology, ML, data science, quantum computing. In my first year I jumped into many domains, sometimes beyond the syllabus.</p>
        <p>By my 4th semester, I realised that falling is part of learning. I learned to pick myself up and work with renewed motivation.</p>
        <p>I am committed to building my future in CS through programming, system design, and problem-solving — valuing teamwork, innovation, and continuous learning.</p>
        <h3>Key Objectives:</h3>
        <ul>
          <li>Excel in programming, system design, and problem-solving</li>
          <li>Advance skills in AI, ML, and Cybersecurity</li>
          <li>Develop innovative solutions in Hardware, IoT, and Robotics</li>
          <li>Contribute to research and publish in cutting-edge technology</li>
          <li>Integrate complex systems for intelligent automation</li>
          <li>Foster collaboration and continuous learning</li>
        </ul>
      </div>
    </div>
  </div>
</div>
</section>

<!-- Page 6: Visits -->
<section class="page" id="page-6">
<div class="container">
  <div class="visit-wrap">
    <h1 class="page-title">Portfolio Visits</h1>
    <div class="visit-card">
      <div class="visit-icon">👁️</div>
      <div class="visit-info">
        <h2>Total Portfolio Visits</h2>
        <div class="visit-count" id="visitCount">{visits}</div>
        <p>Counter is tracked server-side by the Rust process (in-memory).</p>
      </div>
    </div>
    <button class="btn btn-sec" id="resetCounter">Reset Counter</button>
    <p class="reset-note">Calls <code>POST /api/visits/reset</code> on the Rust server</p>
  </div>
</div>
</section>

<!-- Page 7: Future Plans -->
<section class="page" id="page-7">
<div class="container">
  <h1 class="page-title">Future Plans — Next 1–2 Years</h1>
  <div class="plans-wrap">
    <div class="plans-tl">{plans}</div>
  </div>
</div>
</section>

<!-- Page 9: Contact -->
<section class="page" id="page-9">
<div class="container">
  <h1 class="page-title">Contact Me</h1>
  <div class="two-col">
    <div class="col">
      <div class="contact-form-section">
        <h2>Send Message</h2>
        <div id="formSuccess" class="msg-success" style="display:none"></div>
        <div id="formError" class="msg-error" style="display:none"></div>
        <div class="form-group"><label class="form-label">Full Name</label>
          <input type="text" class="form-control" id="cf-name" placeholder="Your name"></div>
        <div class="form-group"><label class="form-label">Email Address</label>
          <input type="email" class="form-control" id="cf-email" placeholder="your@email.com"></div>
        <div class="form-group"><label class="form-label">Subject</label>
          <input type="text" class="form-control" id="cf-subject" placeholder="Message subject"></div>
        <div class="form-group"><label class="form-label">Message</label>
          <textarea class="form-control" id="cf-message" rows="5" placeholder="Your message..."></textarea></div>
        <button class="btn btn-primary full-width" id="cf-submit">Send Message</button>
      </div>
    </div>
    <div class="col">
      <div class="contact-info-section">
        <h2>Contact Information</h2>
        <div class="contact-list">
          <div class="contact-item"><span class="ci-icon">📧</span><div><h4>Email</h4><p>{email}</p></div></div>
          <div class="contact-item"><span class="ci-icon">📱</span><div><h4>Phone</h4><p>{phone}</p></div></div>
          <div class="contact-item"><span class="ci-icon">📍</span><div><h4>Location</h4><p>{location}</p></div></div>
        </div>
        <h3>Connect with me</h3>
        <div class="social-list">
          <a href="{linkedin}" class="social-link" target="_blank" rel="noopener">🔗 LinkedIn</a>
          <a href="{github}" class="social-link" target="_blank" rel="noopener">🐙 GitHub</a>
          <a href="{twitter}" class="social-link" target="_blank" rel="noopener">🐦 Twitter / X</a>
        </div>
      </div>
    </div>
  </div>
</div>
</section>

</main>
<script>{js}</script>
</body>
</html>"#,
        css = CSS, js = JS,
        projects = projects_html, exp = exp_html,
        dev_skills = dev_skills, dev2_skills = dev2_skills,
        courses = courses_html, plans = plans_html,
        degree = esc(&p.education.degree), college = esc(&p.education.college), duration = esc(&p.education.duration),
        visits = visits,
        email = esc(&c.email), phone = esc(&c.phone), location = esc(&c.location),
        linkedin = c.linkedin, github = c.github, twitter = c.twitter,
    )
}

// ─── Embedded CSS ─────────────────────────────────────────────────────────────
const CSS: &str = r#"
:root{
  --bg:#fcfcf9;--surface:#fff;--text:#13343b;--text2:#626c71;
  --primary:#21808d;--primary-h:#1d7480;--primary-btn:#fcfcf9;
  --border:rgba(94,82,64,.2);--card-border:rgba(94,82,64,.12);
  --sec:rgba(94,82,64,.12);--sec-h:rgba(94,82,64,.2);
  --error:#c0152f;--success:#21808d;--warning:#a84b2f;
  --shadow-sm:0 1px 3px rgba(0,0,0,.06);--shadow-md:0 4px 12px rgba(0,0,0,.08);
  --r:8px;--r-lg:12px;--r-full:999px;
  --dur:.25s;--ease:cubic-bezier(.16,1,.3,1);
}
@media(prefers-color-scheme:dark){:root{
  --bg:#1f2121;--surface:#262828;--text:#f5f5f5;--text2:rgba(167,169,169,.75);
  --primary:#32b8c6;--primary-h:#2da6b2;--primary-btn:#13343b;
  --border:rgba(119,124,124,.3);--card-border:rgba(119,124,124,.2);
  --sec:rgba(119,124,124,.15);--sec-h:rgba(119,124,124,.25);
  --error:#ff5459;--success:#32b8c6;
}}
*,*::before,*::after{box-sizing:border-box}
html{font:14px/1.5 "Inter",-apple-system,BlinkMacSystemFont,"Segoe UI",Roboto,sans-serif;
  color:var(--text);background:var(--bg);-webkit-font-smoothing:antialiased}
body{margin:0}
h1,h2,h3,h4{margin:0;font-weight:600;line-height:1.25;color:var(--text)}
h1{font-size:1.9rem}h2{font-size:1.45rem}h3{font-size:1.15rem}
p{margin:0 0 12px}
a{color:var(--primary);text-decoration:none;transition:color var(--dur)}
a:hover{color:var(--primary-h)}
ul{padding-left:20px;color:var(--text2);font-size:.875rem}
ul li{margin-bottom:6px}
code{font-family:ui-monospace,Menlo,monospace;font-size:.8rem;
  background:var(--sec);padding:1px 5px;border-radius:4px}

/* Navbar */
.navbar{position:fixed;inset:0 0 auto;background:var(--surface);
  border-bottom:1px solid var(--border);z-index:1000;box-shadow:var(--shadow-sm)}
.nav-wrap{max-width:1200px;margin:0 auto;padding:0 16px;display:flex;
  align-items:center;justify-content:space-between;height:64px}
.nav-brand{font-weight:700;font-size:1rem;color:var(--primary);display:flex;align-items:center;gap:8px}
.rust-badge{background:#b7410e;color:#fff;font-size:.65rem;padding:2px 7px;
  border-radius:var(--r-full);font-weight:700;letter-spacing:.05em}
.nav-menu{display:flex;gap:6px;flex-wrap:wrap}
.nav-item{padding:5px 11px;border:none;background:transparent;color:var(--text2);
  font-size:.8rem;border-radius:var(--r);cursor:pointer;font-family:inherit;
  transition:all var(--dur) var(--ease)}
.nav-item:hover{background:var(--sec);color:var(--text)}
.nav-item.active{background:var(--primary);color:var(--primary-btn);font-weight:600}
.nav-toggle{display:none;background:none;border:none;font-size:1.4rem;
  cursor:pointer;color:var(--text);padding:4px}

/* Pages */
.main{margin-top:64px;min-height:calc(100vh - 64px)}
.page{display:none;padding:32px 0;animation:fi .3s var(--ease)}
.page.active{display:block}
@keyframes fi{from{opacity:0;transform:translateY(8px)}to{opacity:1;transform:none}}
.page-title{margin-bottom:28px;color:var(--primary)}
.container{max-width:1024px;margin:0 auto;padding:0 16px}

/* Grid */
.two-col{display:grid;grid-template-columns:1fr 1fr;gap:32px}
.home-cols{align-items:center}
.col{}

/* Avatar */
.center-col{display:flex;justify-content:center}
.avatar-wrap{text-align:center}
.avatar{width:160px;height:160px;border-radius:50%;background:linear-gradient(135deg,var(--primary),#1a6873);
  display:flex;align-items:center;justify-content:center;font-size:2.8rem;font-weight:700;
  color:#fff;margin:0 auto;box-shadow:var(--shadow-md)}
.avatar-caption{margin-top:12px;color:var(--text2);font-size:.9rem}
.avatar-badge{display:inline-block;margin-top:8px;background:rgba(183,65,14,.1);
  border:1px solid rgba(183,65,14,.3);color:#b7410e;padding:4px 14px;
  border-radius:var(--r-full);font-size:.75rem;font-weight:600}

/* Intro */
.intro{padding:8px 0}
.name-title{font-size:2.2rem;margin-bottom:6px}
.prof-title{font-size:1.05rem;color:var(--primary);font-weight:400;margin-bottom:20px}
.intro-text p{color:var(--text2);font-size:.9rem}
.highlights{display:flex;flex-direction:column;gap:10px;margin:20px 0}
.hl-item{background:rgba(59,130,246,.08);padding:10px 14px;border-radius:var(--r);font-size:.85rem}
.rust-hl{color:#b7410e;font-weight:700}
.powered{display:inline-flex;align-items:center;gap:8px;
  background:rgba(183,65,14,.08);border:1px solid rgba(183,65,14,.25);
  padding:7px 14px;border-radius:var(--r-full);font-size:.8rem;color:#b7410e;margin-top:14px}

/* Section title */
.sec-title{margin-bottom:18px;font-size:1.1rem;padding-bottom:7px;border-bottom:2px solid var(--primary)}

/* Projects */
.proj-list{display:flex;flex-direction:column;gap:16px}
.project-card{background:var(--surface);border:1px solid var(--card-border);
  border-radius:var(--r-lg);padding:18px;box-shadow:var(--shadow-sm);
  transition:box-shadow var(--dur);border-left:4px solid var(--success)}
.project-card:hover{box-shadow:var(--shadow-md)}
.project-card.wip{border-left-color:var(--primary)}
.project-card h3{font-size:1rem;margin-bottom:6px}
.project-card p{color:var(--text2);font-size:.85rem;margin-bottom:10px}
.tech-tags{display:flex;flex-wrap:wrap;gap:5px;margin-bottom:10px}
.tech-tag{background:rgba(34,197,94,.1);border:1px solid rgba(34,197,94,.2);
  color:var(--text);padding:2px 8px;border-radius:var(--r-full);font-size:.72rem;font-weight:500}
.progress-indicator{margin-top:10px}
.progress-bar{background:var(--sec);border-radius:var(--r-full);height:7px;overflow:hidden}
.progress-fill{height:100%;background:var(--primary);border-radius:var(--r-full);transition:width .7s var(--ease)}
.progress-text{font-size:.72rem;color:var(--text2);margin-top:3px;display:block}

/* Timeline */
.timeline{position:relative;padding-left:28px}
.timeline::before{content:'';position:absolute;left:6px;top:0;bottom:0;width:2px;background:var(--border)}
.timeline-item{position:relative;margin-bottom:28px}
.tl-dot{position:absolute;left:-23px;top:4px;width:12px;height:12px;
  background:var(--primary);border-radius:50%;border:2px solid var(--bg);
  box-shadow:0 0 0 2px var(--primary)}
.tl-content{background:var(--surface);border:1px solid var(--card-border);
  border-radius:var(--r-lg);padding:16px;box-shadow:var(--shadow-sm)}
.tl-content h3{font-size:.95rem;margin-bottom:3px}
.tl-org{color:var(--primary);font-size:.8rem;font-weight:500;margin-bottom:3px}
.tl-period{color:var(--text2);font-size:.72rem;margin-bottom:8px}
.tl-content p{color:var(--text2);font-size:.85rem;margin:0}

/* Achievement */
.ach-card{display:flex;gap:14px;align-items:flex-start;background:var(--surface);
  border:1px solid var(--card-border);border-radius:var(--r-lg);padding:18px;box-shadow:var(--shadow-sm)}
.ach-icon{font-size:2rem}
.ach-card h3{font-size:.95rem;margin-bottom:4px}
.ach-card p{color:var(--text2);font-size:.85rem;margin:0}

/* Skills */
.skills-list{display:flex;flex-direction:column;gap:14px}
.skill-item{background:var(--surface);border:1px solid var(--card-border);
  border-radius:var(--r);padding:14px;box-shadow:var(--shadow-sm)}
.skill-item.developing{border-left:3px solid #f59e0b}
.skill-row{display:flex;justify-content:space-between;margin-bottom:7px}
.skill-name{font-weight:500;font-size:.85rem}
.skill-pct{color:var(--primary);font-weight:700;font-size:.85rem}
.skill-bar{background:var(--sec);border-radius:var(--r-full);height:7px;overflow:hidden}
.skill-fill{height:100%;background:linear-gradient(90deg,var(--primary),#32b8c6);border-radius:var(--r-full);transition:width .7s}
.skill-fill.dev-fill{background:linear-gradient(90deg,var(--primary),#f59e0b)}

/* Academic */
.edu-card{background:var(--surface);border:1px solid var(--card-border);
  border-radius:var(--r-lg);padding:18px;box-shadow:var(--shadow-sm);margin-top:14px}
.edu-card h3{color:var(--primary);margin-bottom:10px;font-size:1rem}
.edu-card p{font-size:.85rem;color:var(--text2);margin-bottom:6px}
.courses-grid{display:grid;grid-template-columns:1fr 1fr;gap:7px;margin-top:14px}
.course-item{background:rgba(6,182,212,.08);padding:8px 12px;
  border-radius:var(--r);font-size:.8rem;text-align:center}
.objective-card{background:var(--surface);border:1px solid var(--card-border);
  border-radius:var(--r-lg);padding:20px;box-shadow:var(--shadow-sm)}
.objective-card p{color:var(--text2);font-size:.85rem}
.objective-card h3{margin:14px 0 8px;font-size:.95rem}

/* Visit counter */
.visit-wrap{text-align:center;max-width:560px;margin:0 auto}
.visit-card{display:flex;align-items:center;justify-content:center;gap:24px;
  background:var(--surface);border:1px solid var(--card-border);
  border-radius:var(--r-lg);padding:32px;margin:20px 0;box-shadow:var(--shadow-sm)}
.visit-icon{font-size:3rem}
.visit-info h2{margin-bottom:6px}
.visit-count{font-size:4rem;font-weight:800;color:var(--primary);line-height:1}
.visit-info p{color:var(--text2);font-size:.8rem;margin-top:8px}
.reset-note{font-size:.75rem;color:var(--text2);margin-top:8px}

/* Plans */
.plans-wrap{max-width:680px;margin:0 auto}
.plans-tl{position:relative;padding-left:28px}
.plans-tl::before{content:'';position:absolute;left:6px;top:0;bottom:0;width:2px;background:var(--border)}
.plan-item{position:relative;background:var(--surface);border:1px solid var(--card-border);
  border-left:4px solid var(--primary);border-radius:var(--r-lg);
  padding:18px;margin:0 0 20px 14px;box-shadow:var(--shadow-sm)}
.plan-dot{position:absolute;left:-40px;top:14px;width:14px;height:14px;
  background:var(--primary);border-radius:50%;border:3px solid var(--bg);
  box-shadow:0 0 0 2px var(--primary)}
.plan-content h3{font-size:.95rem;margin-bottom:4px}
.plan-period{color:var(--text2);font-size:.72rem;margin-bottom:8px}
.plan-content p{color:var(--text2);font-size:.85rem;margin:0}

/* Contact */
.contact-form-section{background:var(--surface);border:1px solid var(--card-border);
  border-radius:var(--r-lg);padding:24px;box-shadow:var(--shadow-sm)}
.contact-form-section h2{margin-bottom:18px}
.contact-info-section{padding:8px 24px}
.contact-list{margin-bottom:24px}
.contact-item{display:flex;align-items:center;gap:14px;padding:14px;
  background:rgba(6,182,212,.08);border-radius:var(--r);margin-bottom:14px}
.ci-icon{font-size:1.4rem}
.contact-item h4{margin:0 0 2px;font-size:.9rem}
.contact-item p{margin:0;color:var(--text2);font-size:.83rem}
.social-list{display:flex;flex-direction:column;gap:10px;margin-top:14px}
.social-link{display:flex;align-items:center;gap:10px;padding:10px 14px;
  background:var(--sec);border-radius:var(--r);color:var(--text);
  text-decoration:none;transition:all var(--dur) var(--ease);font-size:.88rem}
.social-link:hover{background:var(--sec-h);transform:translateX(4px)}

/* Forms */
.form-group{margin-bottom:14px}
.form-label{display:block;font-size:.8rem;font-weight:500;margin-bottom:5px}
.form-control{display:block;width:100%;padding:8px 12px;font-size:.88rem;
  color:var(--text);background:var(--bg);border:1px solid var(--border);
  border-radius:var(--r);font-family:inherit;transition:border-color var(--dur)}
.form-control:focus{border-color:var(--primary);outline:2px solid var(--primary);outline-offset:-2px}
.form-control.error{border-color:var(--error)}
textarea.form-control{resize:vertical}

/* Buttons */
.btn{display:inline-flex;align-items:center;justify-content:center;
  padding:8px 18px;border-radius:var(--r);font-size:.88rem;font-weight:500;
  cursor:pointer;border:none;font-family:inherit;transition:all var(--dur) var(--ease)}
.btn-primary{background:var(--primary);color:var(--primary-btn)}
.btn-primary:hover{background:var(--primary-h)}
.btn-sec{background:var(--sec);color:var(--text)}
.btn-sec:hover{background:var(--sec-h)}
.full-width{width:100%}
.btn:disabled{opacity:.5;cursor:not-allowed}

/* Messages */
.msg-success,.msg-error{padding:10px 14px;border-radius:var(--r);font-size:.85rem;margin-bottom:14px}
.msg-success{background:rgba(33,128,141,.12);color:var(--success)}
.msg-error{background:rgba(192,21,47,.1);color:var(--error)}

/* Responsive */
@media(max-width:768px){
  .nav-menu{display:none;position:absolute;top:100%;left:0;right:0;
    background:var(--surface);flex-direction:column;padding:14px;
    border-top:1px solid var(--border);box-shadow:var(--shadow-md)}
  .nav-menu.active{display:flex}
  .nav-toggle{display:block}
  .nav-item{width:100%;text-align:left;padding:10px 12px;margin-bottom:4px}
  .two-col,.home-cols{grid-template-columns:1fr;gap:20px}
  .avatar{width:120px;height:120px;font-size:2.2rem}
  .name-title{font-size:1.8rem}
  .courses-grid{grid-template-columns:1fr}
}
@media(max-width:480px){
  .container{padding:0 12px}
  .visit-count{font-size:2.5rem}
  .visit-card{flex-direction:column;padding:20px}
  .contact-item{flex-direction:column;text-align:center}
}
"#;

// ─── Embedded JS ──────────────────────────────────────────────────────────────
const JS: &str = r#"
// SPA Navigation
const navMenu = document.getElementById('navMenu');
const navToggle = document.getElementById('navToggle');

function showPage(num) {
  document.querySelectorAll('.page').forEach(p => p.classList.remove('active'));
  const t = document.getElementById('page-' + num);
  if (t) t.classList.add('active');
  document.querySelectorAll('.nav-item').forEach(el => {
    el.classList.toggle('active', Number(el.dataset.page) === Number(num));
  });
  if (window.innerWidth < 800) navMenu.classList.remove('active');
  history.replaceState(null, '', '#page-' + num);
}

document.querySelectorAll('.nav-item').forEach(el =>
  el.addEventListener('click', () => showPage(el.dataset.page)));

navToggle.addEventListener('click', () => navMenu.classList.toggle('active'));

document.addEventListener('click', e => {
  if (navMenu.classList.contains('active') &&
      !navMenu.contains(e.target) && !navToggle.contains(e.target))
    navMenu.classList.remove('active');
});

// Animate name title
function animateName() {
  const el = document.querySelector('.name-title');
  if (!el) return;
  const txt = el.textContent;
  el.innerHTML = '';
  [...txt].forEach((ch, i) => {
    const s = document.createElement('span');
    s.textContent = ch; s.style.display = 'inline-block';
    s.style.opacity = '0'; s.style.transform = 'translateY(18px)';
    s.style.transition = `opacity .3s ${(i*.04).toFixed(2)}s, transform .4s ${(i*.04).toFixed(2)}s`;
    el.appendChild(s);
  });
  setTimeout(() => el.querySelectorAll('span').forEach(s => {
    s.style.opacity = '1'; s.style.transform = 'none';
  }), 100);
}

function animateSubtitle() {
  const el = document.querySelector('.prof-title');
  if (!el) return;
  Object.assign(el.style, {opacity:'0', transform:'translateY(12px)',
    transition:'opacity .7s .55s, transform .5s .55s'});
  setTimeout(() => Object.assign(el.style, {opacity:'1', transform:'none'}), 100);
}

// Visit counter API
async function fetchVisits() {
  try {
    const r = await fetch('/api/visits');
    const d = await r.json();
    const el = document.getElementById('visitCount');
    if (el) el.textContent = d.visits;
  } catch(_) {}
}

document.getElementById('resetCounter')?.addEventListener('click', async () => {
  try {
    const r = await fetch('/api/visits/reset', {method:'POST'});
    const d = await r.json();
    const el = document.getElementById('visitCount');
    if (el) el.textContent = d.visits;
  } catch(_) {}
});

// Contact form via Rust API
function validEmail(e) { return /\S+@\S+\.\S+/.test(e); }

document.getElementById('cf-submit')?.addEventListener('click', async () => {
  const name = document.getElementById('cf-name');
  const email = document.getElementById('cf-email');
  const subject = document.getElementById('cf-subject');
  const message = document.getElementById('cf-message');
  const okEl = document.getElementById('formSuccess');
  const errEl = document.getElementById('formError');
  const btn = document.getElementById('cf-submit');

  [name,email,subject,message].forEach(el => el?.classList.remove('error'));
  okEl.style.display = errEl.style.display = 'none';

  let valid = true;
  if (!name.value.trim())          { name.classList.add('error');    valid=false; }
  if (!validEmail(email.value))    { email.classList.add('error');   valid=false; }
  if (!subject.value.trim())       { subject.classList.add('error'); valid=false; }
  if (!message.value.trim())       { message.classList.add('error'); valid=false; }

  if (!valid) {
    errEl.textContent = 'Please fill in all fields correctly.';
    errEl.style.display = 'block'; return;
  }

  btn.disabled = true; btn.textContent = 'Sending…';
  try {
    const r = await fetch('/api/contact', {
      method:'POST',
      headers:{'Content-Type':'application/json'},
      body: JSON.stringify({
        name:name.value.trim(), email:email.value.trim(),
        subject:subject.value.trim(), message:message.value.trim()
      })
    });
    const d = await r.json();
    if (d.success) {
      okEl.textContent = d.message; okEl.style.display = 'block';
      [name,email,subject,message].forEach(el => el.value='');
      setTimeout(() => okEl.style.display='none', 4000);
    } else {
      errEl.textContent = d.message; errEl.style.display='block';
    }
  } catch(_) {
    errEl.textContent='Network error. Please try again.'; errEl.style.display='block';
  } finally {
    btn.disabled=false; btn.textContent='Send Message';
  }
});

// Init
window.addEventListener('DOMContentLoaded', () => {
  animateName(); animateSubtitle();
  const h = window.location.hash;
  if (h) { const n=parseInt(h.replace('#page-','')); if(n>=1&&n<=9) showPage(n); }
});
"#;

// ─── HTTP helpers ─────────────────────────────────────────────────────────────

fn respond(req: tiny_http::Request, status: u16, ctype: &str, body: String) {
    let response = tiny_http::Response::from_string(body)
        .with_status_code(status)
        .with_header(
            tiny_http::Header::from_bytes(&b"Content-Type"[..], ctype.as_bytes()).unwrap()
        );
    let _ = req.respond(response);
}

fn read_body(req: &mut tiny_http::Request) -> String {
    let mut body = String::new();
    req.as_reader().read_to_string(&mut body).unwrap_or(0);
    body
}

// ─── Main ─────────────────────────────────────────────────────────────────────

fn main() {
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".into());
    let addr = format!("0.0.0.0:{}", port);
    let server = tiny_http::Server::http(&addr).expect("Failed to bind");
    println!(" Praveen Kumar Portfolio — Rust Server");
    println!(" Running at http://127.0.0.1:{}", port);
    println!("   GET  /              → Portfolio (SSR HTML)");
    println!("   GET  /api/visits    → Visit count JSON");
    println!("   POST /api/visits/reset → Reset counter");
    println!("   POST /api/contact   → Contact form");

    let portfolio = Arc::new(data());
    let state = Arc::new(State { visits: Mutex::new(0), data: (*portfolio).clone() });

    for mut req in server.incoming_requests() {
        let st = Arc::clone(&state);
        thread::spawn(move || {
            let method = req.method().to_string();
            let url = req.url().to_string();
            let path = url.split('?').next().unwrap_or("/").to_string();

            match (method.as_str(), path.as_str()) {
                ("GET", "/") | ("GET", "") => {
                    let mut v = st.visits.lock().unwrap();
                    *v += 1; let visits = *v; drop(v);
                    let html = render_page(&st.data, visits);
                    respond(req, 200, "text/html; charset=utf-8", html);
                }
                ("GET", "/api/visits") => {
                    let v = st.visits.lock().unwrap();
                    respond(req, 200, "application/json", json!({"visits": *v}).to_string());
                }
                ("POST", "/api/visits/reset") => {
                    let mut v = st.visits.lock().unwrap();
                    *v = 0;
                    respond(req, 200, "application/json", json!({"visits": 0}).to_string());
                }
                ("POST", "/api/contact") => {
                    let body = read_body(&mut req);
                    match serde_json::from_str::<ContactForm>(&body) {
                        Ok(f) => {
                            let name = f.name.trim().to_string();
                            let email = f.email.trim().to_string();
                            let subject = f.subject.trim().to_string();
                            let message = f.message.trim().to_string();
                            if name.is_empty() || subject.is_empty() || message.is_empty() {
                                respond(req, 400, "application/json",
                                    json!({"success":false,"message":"All fields are required."}).to_string());
                            } else if !email.contains('@') || !email.contains('.') {
                                respond(req, 400, "application/json",
                                    json!({"success":false,"message":"Invalid email address."}).to_string());
                            } else {
                                println!("[Contact] {} <{}> | {} | {}", name, email, subject, message);
                                respond(req, 200, "application/json",
                                    json!({"success":true,"message":"Message received! (Demo mode)"}).to_string());
                            }
                        }
                        Err(_) => {
                            respond(req, 400, "application/json",
                                json!({"success":false,"message":"Invalid request body."}).to_string());
                        }
                    }
                }
                _ => {
                    respond(req, 404, "text/plain", "404 Not Found".into());
                }
            }
        });
    }
}
