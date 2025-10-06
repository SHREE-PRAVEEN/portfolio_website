// SPA Navigation
const NAV_ITEM_CLASS = 'nav-item';
const PAGE_CLASS = 'page';
const ACTIVE_CLASS = 'active';
const navMenu = document.getElementById('navMenu');
const navToggle = document.getElementById('navToggle');
let currentPage = 1;

function showPage(pageNum) {
  // Hide all pages
  const pages = document.getElementsByClassName(PAGE_CLASS);
  for (let p of pages) {
    p.classList.remove(ACTIVE_CLASS);
  }
  // Show target page
  const page = document.getElementById(`page-${pageNum}`);
  if (page) page.classList.add(ACTIVE_CLASS);

  // Handle active nav
  const navItems = document.getElementsByClassName(NAV_ITEM_CLASS);
  for (let item of navItems) {
    item.classList.remove(ACTIVE_CLASS);
    if (Number(item.dataset.page) === Number(pageNum)) {
      item.classList.add(ACTIVE_CLASS);
    }
  }

  // If mobile menu, close it when navigating
  if (window.innerWidth < 800) {
    navMenu.classList.remove('active');
  }

  currentPage = pageNum;
}

function setupNavigation() {
  const navItems = document.getElementsByClassName(NAV_ITEM_CLASS);
  for (let item of navItems) {
    item.addEventListener('click', (e) => {
      showPage(item.dataset.page);
    });
  }
  navToggle.addEventListener('click', () => {
    navMenu.classList.toggle('active');
  });
}

// Animated Name Title (Page 1)
function animateTitle() {
  const name = document.querySelector('.name-title');
  if (!name) return;

  // Animate each letter for entry effect
  const text = name.textContent;
  name.innerHTML = '';
  for (let i = 0; i < text.length; i++) {
    let span = document.createElement('span');
    span.textContent = text[i];
    span.style.opacity = '0';
    span.style.transition = `opacity 0.3s ${(i * 0.04).toFixed(2)}s, transform 0.5s ${(i * 0.04).toFixed(2)}s`;
    span.style.transform = 'translateY(24px)';
    name.appendChild(span);
  }
  setTimeout(() => {
    name.querySelectorAll('span').forEach((span, idx) => {
      span.style.opacity = '1';
      span.style.transform = 'translateY(0)';
    });
  }, 200);
}

function animateSubTitle() {
  const subtitle = document.querySelector('.professional-title');
  if (!subtitle) return;
  subtitle.style.opacity = 0;
  subtitle.style.transform = 'translateY(16px)';
  setTimeout(() => {
    subtitle.style.transition = 'opacity 0.7s 0.7s, transform 0.5s 0.7s';
    subtitle.style.opacity = '1';
    subtitle.style.transform = 'translateY(0)';
  }, 700);
}

// Page 6: Hit Counter using localStorage
function initializeCounter() {
  const counter = document.getElementById('visitCount');
  if (!counter) return;
  let visits = 0;
  try {
    // Ensure localStorage is available
    visits = localStorage.getItem('portfolioVisits');
    visits = visits ? parseInt(visits, 10) : 0;
    // Increment for current visit
    visits += 1;
    localStorage.setItem('portfolioVisits', visits);
  } catch (e) {
    // Graceful fallback
    visits = 'N/A';
  }
  counter.textContent = visits;
}

function setupCounterReset() {
  const resetBtn = document.getElementById('resetCounter');
  if (resetBtn) {
    resetBtn.addEventListener('click', () => {
      localStorage.setItem('portfolioVisits', 0);
      const counter = document.getElementById('visitCount');
      if (counter) counter.textContent = '0';
    });
  }
}

// Page 9: Contact Form Validation
function setupContactForm() {
  const form = document.getElementById('contactForm');
  if (!form) return;
  form.addEventListener('submit', function(e) {
    e.preventDefault();
    let valid = true;
    // Validate each field
    const name = document.getElementById('name');
    const email = document.getElementById('email');
    const subject = document.getElementById('subject');
    const message = document.getElementById('message');

    if (!name.value.trim()) valid = false;
    if (!validateEmail(email.value)) valid = false;
    if (!subject.value.trim()) valid = false;
    if (!message.value.trim()) valid = false;

    if (valid) {
      // Show a success message (simulate)
      showSuccessMessage(form);
    } else {
      // Mark invalid fields
      [name, email, subject, message].forEach(input => {
        if (!input.value.trim()) input.classList.add('error');
        else if (input.classList.contains('error')) input.classList.remove('error');
      });
      if (!validateEmail(email.value)) email.classList.add('error');
    }
  });
}

function validateEmail(email) {
  // Basic email validation
  return /\S+@\S+\.\S+/.test(email);
}

function showSuccessMessage(form) {
  let msg = document.createElement('div');
  msg.className = 'success-message show';
  msg.textContent = 'Message sent successfully! (Demo only)';
  form.prepend(msg);
  setTimeout(() => {
    msg.classList.remove('show');
    msg.remove();
    form.reset();
  }, 2000);
}

// Mobile: collapse menu on outside click
function setupOutsideMenuClose() {
  document.addEventListener('click', function(e) {
    if (
      navMenu.classList.contains('active') &&
      navMenu !== e.target &&
      !navMenu.contains(e.target) &&
      navToggle !== e.target &&
      !navToggle.contains(e.target)
    ) {
      navMenu.classList.remove('active');
    }
  });
}

// On page load:
document.addEventListener('DOMContentLoaded', function () {
  setupNavigation();
  setupOutsideMenuClose();
  animateTitle();
  animateSubTitle();
  initializeCounter();
  setupCounterReset();
  setupContactForm();

  // If hash nav, show correct page
  if (window.location.hash) {
    const hashPage = parseInt(window.location.hash.replace('#page-', ''), 10);
    if (hashPage && hashPage >=1 && hashPage <=9) {
      showPage(hashPage);
    }
  }
});

// Optional: Update hash for SPA navigation
function setHash(pageNum) {
  history.replaceState(null, '', `#page-${pageNum}`);
}

// Attach to page navigation
(function attachHashUpdate() {
  const navItems = document.getElementsByClassName(NAV_ITEM_CLASS);
  for (let item of navItems) {
    item.addEventListener('click', function() {
      setHash(item.dataset.page);
    });
  }
})();