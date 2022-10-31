
/// On Scroll
const observer = new IntersectionObserver((entries) => {
    entries.forEach((entry) => {
        if (entry.isIntersecting){
            entry.target.classList.add('show');
        } else{
            entry.target.classList.remove('show');
        }
    });
});

const hiddenElements = document.querySelectorAll('.hidden');
hiddenElements.forEach((el) => observer.observe(el));

// Click Build
const showBuild = (god) => {
    let old = document.getElementsByClassName("active-build");
    if (old.length > 0){
        old[0].classList.remove("active-build");
    }
    god.classList.add('active-build');
    console.log(god);
    fetch('./' + god.dataset.build + '.html')
        .then((response) => response.text())
        .then(text=> document.getElementById('explain').innerHTML = text);
}



fetch('./test.json')
.then((response) => response.json())
.then((json) => console.log(json));

/// Nav
const toggleNav = () => {
    document.body.dataset.nav = document.body.dataset.nav === "true" ? "false" : "true";
}

// Filter
let role = "";

function filter_class(x){
    let old = document.querySelector("div.selected.class");
    x.classList.toggle("selected");
    if (old !== null){
        old.classList.remove("selected");
    }

    let y = document.querySelector("div.selected.class");
    if (y !== null){
        role = "=" + y.dataset.role;
    } else{
        role = "";
    }

    let filterto = [...document.querySelectorAll('[data-build-role'+ role +']')];
    let allbuilds = document.getElementById('build-picker');
    for (child of allbuilds.children){
        child.classList.remove('hidden-build');
        if (!filterto.includes(child) && filterto.length > 0){
            child.classList.add('hidden-build');
        } 
    }
}

Array.from(document.getElementsByClassName("class"))
    .forEach((item, index) => {
        item.onclick = () => {
            filter_class(item)
        }
    });


function filter_role(x){
    x.classList.toggle("selected");
}

Array.from(document.getElementById("role-picker").querySelector("svg").children)
    .forEach((item, index) => {
        item.onclick = () => {
            filter_role(item)
        }
    });


// Dark modes
const darkButton = document.getElementById('dark_mode');
darkButton.onclick = () => {
    document.body.toggleAttribute("data-dark");
    localStorage.setItem('theme', document.body.dataset.dark);
}
const theme = localStorage.getItem('theme');



// Star effect
let index = 0,
    interval = 1000;

const rand = (min, max) => 
  Math.floor(Math.random() * (max - min + 1)) + min;

const animate = star => {
  star.style.setProperty("--star-left", `${rand(-10, 100)}%`);
  star.style.setProperty("--star-top", `${rand(-40, 80)}%`);

  star.style.animation = "none";
  star.offsetHeight;
  star.style.animation = "";
}

for(const star of document.getElementsByClassName("magic-star")) {
    console.log("test");
  setTimeout(() => {
    animate(star);
    
    setInterval(() => animate(star), 1000);
  }, index++ * (interval / 3))
}

for(const star of document.getElementsByClassName("leaf")) {
    console.log("test2");
  setTimeout(() => {
    animate(star);
    
    setInterval(() => animate(star), 700);
  }, index++ * (700 / 3))
}