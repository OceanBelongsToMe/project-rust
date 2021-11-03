<script>
    import {spring} from 'svelte/motion';

    let coords = spring({x: 50, y: 50}, {
        stiffness: 0.2,
        damping: 1
    });

    let size = spring(10);
</script>

<style>
    svg {
        width: 100%;
        height: 100%;
        position: absolute;
        top: 0;
        left: 0;
    }

    circle {
        fill: #ff3e00
    }
</style>

<div style="position: absolute; right: 1em;">
    <label>
        <h3>stiffness ({coords.stiffness})</h3>
        <input bind:value={coords.stiffness} type="range" min="0" max="1" step="0.01">
    </label>

    <label>
        <h3>damping ({coords.damping})</h3>
        <input bind:value={coords.damping} type="range" min="0" max="1" step="0.01">
    </label>
</div>

<svg
        on:mousemove="{e => coords.set({ x: e.clientX, y: e.clientY })}"
        on:mousedown="{() => size.set(30)}"
        on:mouseup="{() => size.set(10)}"
>
    <path  d="M0 0 L{$coords.x} {$coords.y}" style="stroke:black;stroke-width:2"/>
    <circle cx={$coords.x} cy={$coords.y} r={$size}/>
</svg>
