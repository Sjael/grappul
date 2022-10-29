
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