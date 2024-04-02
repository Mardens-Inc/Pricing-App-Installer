const installLocationInput = $("input#install-location");


$("#select-install-location").on("click", async () => {
    let location = await window.__TAURI__.dialog.open({
        defaultPath: installLocationInput.val(),
        directory: true
    })
    console.log(location)
    if (location === null || location === "") return;
    location = `${location}\\Mardens Inc.\\Pricing App`;
    installLocationInput.val(location);
});

$("#install").on("click", async () => {

    const location = installLocationInput.val();
    const createDesktopShortcut = $("toggle#create-desktop-shortcut").attr("value") === "true";
    const createStartMenuShortcut = $("toggle#create-start-menu-shortcut").attr("value") === "true";
    const startWithWindows = $("toggle#start-with-windows").attr('value') === "true";

    await window.__TAURI__.invoke("install", {
        location: location,
        createDesktopShortcut: createDesktopShortcut,
        createStartMenuShortcut: createStartMenuShortcut,
        startWithWindows: startWithWindows
    });
});

fetch("https://pricing-new.mardens.com/api/clients/versions").then(response => {
    if (!response.ok) {
        serverDown();
    }
}).catch(() => serverDown());

const serverDown = () => {
    $("main").html(`<h1 style="text-align: center;font-size: 5rem;">Server is down</h1>`)
    $("#action-button-row").remove();
};