const body = document.querySelector('body');

// LINK BUTTONS //
const linkButtons = document.querySelectorAll("[link-href]");
linkButtons.forEach((linkbutton)=>{
    linkbutton.addEventListener("click", () => {
        window.location.href = linkbutton.getAttribute("link-href");
    });
});

// DROPDOWN CONTROLLER //
const dropdowns = document.querySelectorAll(".dropdown");
dropdowns.forEach((dropdown) => {
    const dropbtn = dropdown.querySelector(".dropbtn");
    const dropdownMenu = dropdown.querySelector(".dropdown-menu");
    const dropdownItems = dropdown.querySelectorAll(".dropdown-item");
    const hidDropdown = ()=>{
        dropdownMenu.style.visibility = 'hidden';
        dropdownMenu.style.opacity = '0';
    }
    dropbtn.addEventListener("click", ()=>{
        let visible = dropdownMenu.style.visibility === 'visible' ? ['hidden',0]:['visible',1];
        dropdownMenu.style.visibility = visible[0];
        dropdownMenu.style.opacity = visible[1];
    })
    body.addEventListener('click', ()=>{
        hidDropdown();
    });
    dropdown.addEventListener('click', (event) => {
        if (!dropdown.contains(event.target) || Array.from(dropdownItems).includes(event.target)) {
            hidDropdown()
        }
        event.stopPropagation();
    });
    dropbtn.addEventListener('click', () => {
        dropdowns.forEach((otherDropdown) => {
            const otherDropdownMenu = otherDropdown.querySelector('.dropdown-menu');
            if (otherDropdownMenu !== dropdownMenu && otherDropdownMenu.style.visibility === 'visible') {
                otherDropdownMenu.style.visibility = 'hidden';
                otherDropdownMenu.style.opacity = '0';
            }
        });
    });
})
// DIALOG CONTROLLER //
const dialogOpenBtns = document.querySelectorAll("[open-dialog]");
const dialogs = document.querySelectorAll(".dialog");
dialogOpenBtns.forEach((button)=>{
    button.addEventListener("click", ()=>{
        let dialog = document.querySelector(`#${button.getAttribute("open-dialog")}`);
        dialog.open=true;
    })
})
dialogs.forEach((dialog)=>{
    const closeBtn = document.querySelector("[close-btn]");
    const dialogCard = document.querySelector(".card");
    closeBtn.addEventListener('click',()=>{
        dialog.open=false;
    });
    body.addEventListener('click', () => {
        dialog.open = false;
    });
    dialogCard.addEventListener('click', (event) => {
        event.stopPropagation();
    });
})

// FORM CONTROLLER //

/*const forms = document.getElementById('my-form');

form.addEventListener('submit', async (e) => {
    e.preventDefault();

    const url = 'http://exemplo.com/processar';
    const data = new FormData(form);

    const response = await fetch(url, {
        method: 'POST',
        body: data
    });

    console.log(response.status);
});*/


