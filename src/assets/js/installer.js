import {startLoading, stopLoading} from "./loading.js";

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
    startLoading({message: "Installing the app. Please wait..."});
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
    await window.__TAURI__.invoke("start_application", {exe: `${location}/pricing-app.exe`});
    stopLoading();
});

startLoading()
Promise.all([
    (async () => {
        const location = await window.__TAURI__.invoke("get_default_install_location");
        installLocationInput.val(location);
    })(),
    (async () => {
        const response = await fetch("https://pricing-new.mardens.com/api/clients/versions");
        if (!response.ok) {
            $("main").html(`<h1 style="text-align: center;font-size: 5rem;">Server is down</h1>`)
            $("#action-button-row").remove();
        }
    })()
]).then(() => {
    stopLoading();
})