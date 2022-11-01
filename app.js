
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

// Filter Classes
let god_class = "";

function filter_class(x){
    let old = document.getElementById("class-bar").querySelector(".selected");
    x.classList.toggle("selected");
    if (old !== null){
        old.classList.remove("selected");
    }

    let y = document.getElementById("class-bar").querySelector(".selected");
    if (y !== null){
        god_class = "=" + y.dataset.class;
    } else{
        god_class = "";
    }
    filter_both();
}

Array.from(document.getElementById("class-bar").children)
    .forEach((item, index) => {
        item.onclick = () => {
            filter_class(item)
        }
    });


/// Filter Roles
let god_role = "";
function filter_role(x){
    let old = document.querySelector(".selected.role");
    x.classList.toggle("selected");
    if (old !== null){
        old.classList.remove("selected");
    }

    let y = document.querySelector(".selected.role");
    if (y !== null){
        god_role = "=" + y.dataset.role;
    } else{
        god_role = "";
    }
    filter_both()
}

Array.from(document.getElementById("role-bar").querySelector("svg").children)
    .forEach((item, index) => {
        item.onclick = () => {
            filter_role(item)
        }
    });

function filter_both(){
    console.log(god_role)
    console.log(god_class)
    let filterclasses = [...document.querySelectorAll('[data-build-class'+ god_class +']')];
    let filterroles = [...document.querySelectorAll('[data-build-role'+ god_role +']')];
    let intersection = filterclasses.filter(i => filterroles.includes(i));

    let allbuilds = document.getElementById('build-picker');
    for (child of allbuilds.children){
        child.classList.remove('hidden-build');
        if (!intersection.includes(child) && intersection.length > 0){
            child.classList.add('hidden-build');
        } 
    }
}

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