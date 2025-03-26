let buttonsPressed = new Set()
let websocket
let timer
let leftStickData
let intervalId
const hostIP = window.location.host
document.addEventListener("DOMContentLoaded", () => {
    new JoyStick("joystick",
        {
            internalFillColor: "#000000",
            internalStrokeColor: "#000000",
            externalStrokeColor: "#000000",
        }, (stickData) => {
            leftStickData = stickData
        }
    )

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

function establishConnection() {
    if (!intervalId) {
        websocket = new WebSocket(`ws://${location.host}/ws`);
        websocket.onopen = () => {
            console.log("ready")
            intervalId = setInterval(sendInput, 250)
        }
        websocket.onclose = () => {
            closeConnection()
        }
    }
}

function closeConnection() {
    if (intervalId) {
        clearInterval(intervalId)
        intervalId = undefined
    }
    if (websocket) {
        websocket.close()
    }
}

function sendInput() {
    if (!websocket) {
        return
    }
    console.log("Sending")
    const data = JSON.stringify({
        controller_id: document.getElementById("controller-id").value,
        action_ids: Array.from(buttonsPressed),
        left_stick: {
            x: Number(leftStickData.x),
            y: Number(leftStickData.y),
        },
        right_stick: {
            x: 0,
            y: 0,
        },
        triggers: {
            left: 0,
            right: 0,
        }
    })
    console.log(data)
    websocket.send(data)
    buttonsPressed.clear()
}

function queueButton(target) {
    buttonsPressed.add(target.id)
    console.log(buttonsPressed)
}