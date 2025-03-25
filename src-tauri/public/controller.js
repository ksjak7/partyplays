let controllerId = ""
let timer
document.addEventListener("DOMContentLoaded", () => {
    new JoyStick("joystick",
        {
            internalFillColor: "#000000",
            internalStrokeColor: "#000000",
            externalStrokeColor: "#000000",
        }, (stickdata) => {
            if (!timer) {
                timer = true
                setTimeout( () => {
                    console.log(stickdata.x, stickdata.y)
                    timer = false
                }, 500)
            }
    })

    rotation_handler()
})
window.addEventListener("orientationchange", rotation_handler)
window.addEventListener("resize", rotation_handler)

function rotation_handler() {
    const controller = document.getElementById("controller")
    const warning = document.getElementById("warning")

    if (window.innerHeight > window.innerWidth) {
        controller.style = "display: none;"
        warning.style = "display: flex;"
    } else {
        controller.style = "display: flex;"
        warning.style = "display:none;"
    }
}