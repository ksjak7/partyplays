<script lang="ts">
    import type { Shape, ShapeConfig } from "konva/lib/Shape";
    import { Stage as Sta } from "konva/lib/Stage";
    import {
        Stage,
        Layer,
        Circle,
        type KonvaDragTransformEvent,
    } from "svelte-konva";

    let inputReceived = $state(false);
    let thumb_lx = $state(0);
    let thumb_ly = $state(0);

    const joystickSize = 300;
    const centerJoystickPosition = joystickSize / 2;
    const thumbStickConfig = {
        id: "circle",
        x: centerJoystickPosition,
        y: centerJoystickPosition,
        radius: 40,
        fill: "blue",
        draggable: true,
    };

    function thumbStickDragStart(event: KonvaDragTransformEvent) {
        [thumb_lx, thumb_ly] = getCorrectedPosition(
            event.detail.target.getPosition(),
        );
    }

    function thumbStickDragMove(event: KonvaDragTransformEvent) {
        [thumb_lx, thumb_ly] = getCorrectedPosition(
            event.detail.target.getPosition(),
        );
    }

    function thumbStickDragEnd(event: KonvaDragTransformEvent) {
        [thumb_lx, thumb_ly] = getCorrectedPosition(
            event.detail.target.getPosition(),
        );
        event.detail.target.to({
            x: centerJoystickPosition,
            y: centerJoystickPosition,
            duration: 0,
        });
        thumb_lx = 0;
        thumb_ly = 0;
    }

    function resetEventTargetPosition(eventTarget: Shape<ShapeConfig> | Sta) {}

    function clamp(value: number, min: number, max: number): number {
        return Math.max(Math.min(value, max), min);
    }

    function getCorrectedPosition(position: {
        x: number;
        y: number;
    }): [number, number] {
        return [
            clamp(position.x, 0, joystickSize) - centerJoystickPosition,
            -(clamp(position.y, 0, joystickSize) - centerJoystickPosition),
        ];
    }
</script>

<main>
    <Stage config={{ width: joystickSize, height: joystickSize }}>
        <Layer>
            <Circle
                config={thumbStickConfig}
                on:dragstart={(e) => thumbStickDragStart(e)}
                on:dragmove={(e) => thumbStickDragMove(e)}
                on:dragend={(e) => thumbStickDragEnd(e)}
            />
        </Layer>
    </Stage>
    <p>{thumb_lx}, {thumb_ly}</p>
</main>

<style>
</style>
