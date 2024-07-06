<script setup lang="ts">

import { ref, onMounted, watch, onUnmounted } from 'vue'

interface Point {
  x: number;
  y: number;
}

const touchPad = ref<HTMLDivElement | null>(null);

const point = ref<Point>({ x: 0, y: 0 });

const ws = new WebSocket(`ws://${location.host}/ws?id=123&name=dddf`);

ws.onopen = function () {
  console.log('connect');

  ws.send('hello 123');
};
ws.onmessage = function (e) {
  console.log('message', e.data);
};

function scale(v: number, exponentially_scale = 1, liner_scale = 1) {
  if (!v) return v;
  const isPositive = v > 0;
  let value = Math.abs(v);
  value = value * liner_scale;
  value = Math.pow(value, exponentially_scale);
  value = Math.floor(value + 0.5);

  if (isPositive) {
    return value;
  } else {
    return -value;
  }

}


watch(point, (newPoint, oldPoint) => {
  if (newPoint.x === 0 && newPoint.y === 0 || oldPoint.x === 0 && oldPoint.y === 0) {
    return;
  }
  const deltaPos = { x: newPoint.x - oldPoint.x, y: newPoint.y - oldPoint.y };
  console.log(deltaPos);
  deltaPos.x = Math.floor(scale(deltaPos.x, 1.5, 1.3));
  deltaPos.y = Math.floor(scale(deltaPos.y, 1.5, 1.3));
  ws.send(JSON.stringify(deltaPos));
});

function onTouch(e: TouchEvent) {
  const touch = e.touches[0];
  if (touch) {
    point.value = { x: touch.clientX, y: touch.clientY };
  }
  e.preventDefault();
}

function onTouchEnd(e: TouchEvent) {
  point.value = { x: 0, y: 0 };
}

onMounted(() => {
  if (touchPad.value) {
    touchPad.value.addEventListener('touchstart', onTouch);
    touchPad.value.addEventListener('touchmove', onTouch);
    touchPad.value.addEventListener('touchend', onTouchEnd);
  }
});

onUnmounted(() => {
  if (touchPad.value) {
    touchPad.value.removeEventListener('touchstart', onTouch);
    touchPad.value.removeEventListener('touchmove', onTouch);
    touchPad.value.removeEventListener('touchend', onTouchEnd);
  }
});


</script>

<template>
  <main>

    <div ref="touchPad" class="touch-pad">

    </div>
  </main>
</template>

<style scoped>
.touch-pad {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-color: #f0f0f0;
  overflow: hidden;
}
</style>
