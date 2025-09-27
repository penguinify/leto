// Intro animation for page load


/**
 * Creates and runs the intro animation overlay.
 */
function showIntroAnimation() {
    // Inject keyframes for animation
    const style = document.createElement("style");
    style.textContent = `
           @keyframes introanim {
        0% {
            opacity: 0;
            transform: translate(-150%, -80%) rotate(-45deg);
            filter: blur(10px);
        }

        30% {
            transform: translate(20%, 20%) rotate(15deg);
        }

        60% {
            transform: translate(-5%, -5%) rotate(-4deg);
        }

        100% {
            filter: none;
            transform: translate(0, 0) rotate(0deg);
            opacity: 1;
        }
    }    `;
    document.head.appendChild(style);

    // Create overlay
    const overlay = document.createElement("div");
    overlay.style.cssText = `
        position: fixed;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        z-index: 9999;
        background: black;
        display: flex;
        justify-content: center;
        align-items: center;
        opacity: 1;
    `;

    // Create animated text
    const text = document.createElement("div");
    text.style.cssText = `
        font-size: 8em;
        color: white;
        z-index: 10000;
        display: flex;
        font-family: 'Times New Roman', Times, serif;
    `;

    // Helper to create animated letter
    function createLetter(char, delay = 0) {
        const el = document.createElement("p");
        el.innerText = char;
        el.style.cssText = `
            animation: introanim 1.5s ease-in-out forwards;
            opacity: 0;
            ${delay ? `animation-delay: ${delay}s;` : ""}
        `;
        return el;
    }

    text.appendChild(createLetter("l", 0.2));
    text.appendChild(createLetter("e", 0.3));
    text.appendChild(createLetter("t", 0.4));
    text.appendChild(createLetter("o", 0.5));

    // Credits
    const credits = document.createElement("p");
    credits.innerText = "@penguinify";
    credits.style.cssText = `
        position: absolute;
        bottom: 10px;
        right: 10px;
        color: white;
        font-size: 1em;
        opacity: 0.5;
    `;
    overlay.appendChild(credits);

    overlay.appendChild(text);
    document.body.appendChild(overlay);

    // Animate overlay fade out
    overlay.animate(
        [
            { opacity: 1 },
            { opacity: 0 }
        ],
        {
            duration: 1000,
            delay: 1000,
            fill: "forwards",
            easing: "ease-in-out"
        }
    );

    // Animate text scaling
    text.animate(
        [
            { transform: "scale(1)" },
            { transform: "scale(1.3)", opacity: 0.5 }
        ],
        {
            duration: 800,
            delay: 1200,
            fill: "forwards",
            easing: "ease-in-out"
        }
    );

    // Remove overlay after animation
    setTimeout(() => {
        overlay.remove();
    }, 2500);
}

showIntroAnimation();
