let StickStatus =
{
    xPosition: 0,
    yPosition: 0,
    x: 0,
    y: 0,
};

/**
 * @desc Principal object that draw a joystick, you only need to initialize the object and suggest the HTML container
 * @costructor
 * @param container {String} - HTML object that contains the Joystick
 * @param parameters (optional) - object with following keys:
 *  title {String} (optional) - The ID of canvas (Default value is 'joystick')
 *  width {Int} (optional) - The width of canvas, if not specified is setted at width of container object (Default value is the width of container object)
 *  height {Int} (optional) - The height of canvas, if not specified is setted at height of container object (Default value is the height of container object)
 *  internalFillColor {String} (optional) - Internal color of Stick (Default value is '#00AA00')
 *  internalLineWidth {Int} (optional) - Border width of Stick (Default value is 2)
 *  internalStrokeColor {String}(optional) - Border color of Stick (Default value is '#003300')
 *  externalLineWidth {Int} (optional) - External reference circonference width (Default value is 2)
 *  externalStrokeColor {String} (optional) - External reference circonference color (Default value is '#008000')
 *  autoReturnToCenter {Bool} (optional) - Sets the behavior of the stick, whether or not, it should return to zero position when released (Default value is True and return to zero)
 * @param callback {StickStatus} - 
 */
var JoyStick = (function (container, parameters, callback) {
    parameters = parameters || {};
    console.log(parameters)
    var title = (typeof parameters.title === "undefined" ? "joystick" : parameters.title),
        width = (typeof parameters.width === "undefined" ? 0 : parameters.width),
        height = (typeof parameters.height === "undefined" ? 0 : parameters.height),
        internalFillColor = (typeof parameters.internalFillColor === "undefined" ? "#00AA00" : parameters.internalFillColor),
        internalLineWidth = (typeof parameters.internalLineWidth === "undefined" ? 2 : parameters.internalLineWidth),
        internalStrokeColor = (typeof parameters.internalStrokeColor === "undefined" ? "#003300" : parameters.internalStrokeColor),
        externalLineWidth = (typeof parameters.externalLineWidth === "undefined" ? 2 : parameters.externalLineWidth),
        externalStrokeColor = (typeof parameters.externalStrokeColor === "undefined" ? "#008000" : parameters.externalStrokeColor),
        autoReturnToCenter = (typeof parameters.autoReturnToCenter === "undefined" ? true : parameters.autoReturnToCenter);

    callback = callback || function (StickStatus) { };

    var objContainer = document.getElementById(container);

    objContainer.style.touchAction = "none";

    var canvas = document.createElement("canvas");
    canvas.id = title;
    if (width === 0) { width = objContainer.clientWidth; }
    if (height === 0) { height = objContainer.clientHeight; }
    console.log(width, height)
    canvas.width = width;
    canvas.height = height;
    objContainer.appendChild(canvas);
    var context = canvas.getContext("2d");

    var pressed = 0;
    var circumference = 2 * Math.PI;
    var internalRadius = (canvas.width - ((canvas.width / 2) + 10)) / 2;
    var maxMoveStick = internalRadius + 5;
    var externalRadius = internalRadius + 30;
    var centerX = canvas.width / 2;
    var centerY = canvas.height / 2;
    var directionHorizontalLimitPos = canvas.width / 10;
    var directionHorizontalLimitNeg = directionHorizontalLimitPos * -1;
    var directionVerticalLimitPos = canvas.height / 10;
    var directionVerticalLimitNeg = directionVerticalLimitPos * -1;
    // Used to save current position of stick
    var movedX = centerX;
    var movedY = centerY;

    canvas.addEventListener("touchstart", onTouchStart, false);
    document.addEventListener("touchmove", onTouchMove, false);
    document.addEventListener("touchend", onTouchEnd, false);

    drawExternal();
    drawInternal();

    function drawExternal() {
        context.beginPath();
        context.arc(centerX, centerY, externalRadius, 0, circumference, false);
        context.lineWidth = externalLineWidth;
        context.strokeStyle = externalStrokeColor;
        context.stroke();
    }

    function drawInternal() {
        context.beginPath();
        if (movedX < internalRadius) { movedX = maxMoveStick; }
        if ((movedX + internalRadius) > canvas.width) { movedX = canvas.width - (maxMoveStick); }
        if (movedY < internalRadius) { movedY = maxMoveStick; }
        if ((movedY + internalRadius) > canvas.height) { movedY = canvas.height - (maxMoveStick); }
        context.arc(movedX, movedY, internalRadius, 0, circumference, false);
        // create radial gradient
        var grd = context.createRadialGradient(centerX, centerY, 5, centerX, centerY, 200);
        // Light color
        grd.addColorStop(0, internalFillColor);
        // Dark color
        grd.addColorStop(1, internalStrokeColor);
        context.fillStyle = grd;
        context.fill();
        context.lineWidth = internalLineWidth;
        context.strokeStyle = internalStrokeColor;
        context.stroke();
    }

    let touchId = null;
    function onTouchStart(event) {
        pressed = 1;
        touchId = event.targetTouches[0].identifier;
    }

    function onTouchMove(event) {
        if (pressed === 1 && event.targetTouches[0].target === canvas) {
            movedX = event.targetTouches[0].pageX;
            movedY = event.targetTouches[0].pageY;
            // Manage offset
            if (canvas.offsetParent.tagName.toUpperCase() === "BODY") {
                movedX -= canvas.offsetLeft;
                movedY -= canvas.offsetTop;
            }
            else {
                movedX -= canvas.offsetParent.offsetLeft;
                movedY -= canvas.offsetParent.offsetTop;
            }
            // Delete canvas
            context.clearRect(0, 0, canvas.width, canvas.height);
            // Redraw object
            drawExternal();
            drawInternal();

            // Set attribute of callback
            StickStatus.xPosition = movedX;
            StickStatus.yPosition = movedY;
            StickStatus.x = (100 * ((movedX - centerX) / maxMoveStick)).toFixed();
            StickStatus.y = ((100 * ((movedY - centerY) / maxMoveStick)) * -1).toFixed();
            callback(StickStatus);
        }
    }

    function onTouchEnd(event) {
        if (event.changedTouches[0].identifier !== touchId) return;

        pressed = 0;
        // If required reset position store variable
        if (autoReturnToCenter) {
            movedX = centerX;
            movedY = centerY;
        }
        // Delete canvas
        context.clearRect(0, 0, canvas.width, canvas.height);
        // Redraw object
        drawExternal();
        drawInternal();

        // Set attribute of callback
        StickStatus.xPosition = movedX;
        StickStatus.yPosition = movedY;
        StickStatus.x = (100 * ((movedX - centerX) / maxMoveStick)).toFixed();
        StickStatus.y = ((100 * ((movedY - centerY) / maxMoveStick)) * -1).toFixed();
        callback(StickStatus);
    }

    this.GetWidth = function () {
        return canvas.width;
    };

    this.GetHeight = function () {
        return canvas.height;
    };

    this.GetPosX = function () {
        return movedX;
    };

    this.GetPosY = function () {
        return movedY;
    };

    this.GetX = function () {
        return (100 * ((movedX - centerX) / maxMoveStick)).toFixed();
    };

    this.GetY = function () {
        return ((100 * ((movedY - centerY) / maxMoveStick)) * -1).toFixed();
    };

    this.GetDir = function () {
        return getCardinalDirection();
    };
});