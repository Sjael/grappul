
/// On Scroll
const observer = new IntersectionObserver((entries) => {
    entries.forEach((entry) => {
        console.log(entry)
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
Array.from(document.getElementsByClassName("role"))
    .forEach((item, index) => {
        item.onclick = () => {
            let old = document.getElementsByClassName("selected role");
            if (old.length > 0){
                console.log("old is this")
                console.log(old[0]);
                old[0].classList.remove("selected");
            }
            console.log(item);
            item.classList.add("selected");
            
            
            let filterto = [...document.querySelectorAll('[data-build-role='+ item.dataset.role +']')];
            let allbuilds = document.getElementById('build-picker');
            for (child of allbuilds.children){
                child.classList.remove('hidden-build');
                if (!filterto.includes(child) && filterto.length > 0){
                    child.classList.add('hidden-build');
                } 
            }
        }
    });


// Dark modes
const darkButton = document.getElementById('dark');
const lightButton = document.getElementById('light');
const solarButton = document.getElementById('solar');
const body = document.body;

darkButton.onclick = () => {
    body.classList.replace('light', 'dark');
    localStorage.setItem('theme', 'dark');
}

lightButton.onclick = () => {
    body.classList.replace('dark', 'light');
    localStorage.setItem('theme', 'light');
}

const theme = localStorage.getItem('theme');
if (theme) {
    body.classList.add(theme);
}