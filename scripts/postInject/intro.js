// sends ipc message when the page has finished loading
import { newIpcMessage } from "../shared/ipc";
import Background from "./background.svg";

console.log(Background)

function intro_animation() {

    let keyframes = document.createElement("style");
    keyframes.innerHTML = `
    @keyframes introanim {
        0% {
            opacity: 0;
            transform: translate(-100%, -60%) rotate(-45deg);
            filter: blur(10px);
        }

        30% {
            transform: translate(15%, 15%) rotate(15deg);
        }

        60% {
            transform: translate(-4%, -4%) rotate(-4deg);
            opacity: 1;
        }

        100% {
            filter: none;
            transform: translate(0, 0) rotate(0deg);
            opacity: 1;
        }
    }
    `;

    document.head.appendChild(keyframes);

    let wipe = document.createElement("div");
    wipe.style = `
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background-image: url('data:image/svg+xml;utf8,${encodeURIComponent(Background)}');
    background-size: cover;
    z-index: 9999;
    display: flex;
    justify-content: center;
    align-items: center;
    opacity: 1;
    `

    let text = document.createElement("div");
    text.style = `
    font-size: 8em;
    color: white;
    z-index: 10000;
    display: flex;
    font-family: 'Times New Roman', Times, serif;
    mix-blend-mode: difference;

    `;

    let l = document.createElement("p");
    l.innerText = "l";
    l.style = `
    animation: introanim 1.5s ease-in-out forwards;
    opacity: 0;
    `;
    text.appendChild(l);
    let e = document.createElement("p");
    e.innerText = "e";
    e.style = `
    animation: introanim 1.5s ease-in-out forwards;
    animation-delay: 0.1s;
    opacity: 0;
    `;
    text.appendChild(e);
    let t = document.createElement("p");
    t.innerText = "t";
    t.style = `
    animation: introanim 1.5s ease-in-out forwards;
    animation-delay: 0.2s;
    opacity: 0;
    `;
    text.appendChild(t);
    let o = document.createElement("p");
    o.innerText = "o";
    o.style = `
    animation: introanim 1.5s ease-in-out forwards;
    animation-delay: 0.3s;
    opacity: 0;
    `;
    text.appendChild(o);

    let credits = document.createElement("p");
    credits.innerText = "@penguinify";
    credits.style = `
    position: absolute;
    bottom: 10px;
    right: 10px;
    color: white;
    font-size: 1em;
    opacity: 0.5;
    `;
    wipe.appendChild(credits);

    wipe.appendChild(text);

    wipe.animate([
        {
            opacity: 1
        },
        { opacity: 0 }
    ], {
        duration: 1000,
        delay: 1000,
        fill: "forwards",
        easing: "ease-in-out"
    });

    text.animate([
        {
            transform: "scale(1)"
        },
        {
            transform: "scale(1.3)",
            opacity: 0.5
        }], {
        duration: 800,
        delay: 1200,
        fill: "forwards",
        easing: "ease-in-out"
    });

    document.body.appendChild(wipe);

    setTimeout(() => {
        wipe.remove();
    }, 2000);
}

document.addEventListener("DOMContentLoaded", () => {
    intro_animation();
});
