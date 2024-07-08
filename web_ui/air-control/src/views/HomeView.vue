<script setup lang="ts">

import { ref, onMounted, watch, onUnmounted, reactive } from 'vue'

interface PousePoint {
  x: number;
  y: number;
}

enum MouseState {
  None = 0,
  Move = 1,
  Scroll = 2,
  Drag = 3,
  Click = 4,
  RightClick = 5
}

const touchPad = ref<HTMLDivElement | null>(null);

const mouseDate = reactive<{ state: MouseState, point: PousePoint }>({
  state: MouseState.None,
  point: { x: 0, y: 0 }
});

let scrollJudgeOffset = 0;

const ws = new WebSocket(`ws://${location.host}/ws?id=123&name=dddf`);

ws.onopen = function () {
  console.log('connect');

  ws.send('hello 123');
};
ws.onmessage = function (e) {
  console.log('message', e.data);
};

let clickCheckTime = 0;

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


watch(() => {
  return { ...mouseDate };

}, (newdate, olddate) => {
  if (newdate.state === MouseState.None ||
    (newdate.point.x === 0 && newdate.point.y === 0 || olddate.point.x === 0 && olddate.point.y === 0)) {
    return;
  }

  const newPoint = newdate.point;
  const oldPoint = olddate.point;
  // console.log('watch', newdate);
  if (newdate.state === MouseState.Move) {
    const deltaPos = { x: newPoint.x - oldPoint.x, y: newPoint.y - oldPoint.y };

    const x = Math.floor(scale(deltaPos.x, 1.5, 1.1));
    const y = Math.floor(scale(deltaPos.y, 1.5, 1.1));
    if (x === 0 && y === 0) {
      return;
    }
    ws.send(JSON.stringify({
      pos_type: newdate.state,
      s: 0,
      x,
      y
    }));
  } else if (newdate.state === MouseState.Scroll) {
    const delta = newPoint.y - oldPoint.y;
    scrollJudgeOffset += delta;

    if (Math.abs(scrollJudgeOffset) > 6) {
      const d = {
        pos_type: newdate.state,
        s: Math.round(scrollJudgeOffset / 6),
        x: 0,
        y: 0
      }
      ws.send(JSON.stringify(d));
      scrollJudgeOffset = 0;
    }
  }

});

function onTouch(e: TouchEvent) {
  // console.log('e.touchs', e)
  switch (mouseDate.state) {
    case MouseState.None:
      clickCheckTime = Date.now();

      switch (e.touches.length) {
        case 1:
          mouseDate.state = MouseState.Move;
          break;
        case 2:
          mouseDate.state = MouseState.Scroll;
          break;
        case 3:
          mouseDate.state = MouseState.Drag;
          break;
      }
      handleMouseMove(e.touches);
      break;
    case MouseState.Move:
      switch (e.touches.length) {
        case 1:
          break;
        case 2:
          mouseDate.state = MouseState.Scroll;
          break;
        case 3:
          mouseDate.state = MouseState.Drag;
          break;
      }
      handleMouseMove(e.touches);
      break;
    case MouseState.Scroll:
      switch (e.touches.length) {
        case 1:
          break;
        case 2:
          break;
        case 3:
          mouseDate.state = MouseState.Drag;
          break;
      }
      handleMouseMove(e.touches);
      break;
    case MouseState.Drag:
      break;
  }
  e.preventDefault();
}
function handleMouseMove(touchList: TouchList) {
  if (touchList.length === 0) {
    return;
  }
  // console.log('handleMouseMove', touchList);
  const p0 = touchList[0];
  mouseDate.point = { x: p0.clientX, y: p0.clientY };
}

function onTouchEnd(e: TouchEvent) {
  // check click
  if (Date.now() - clickCheckTime < 100) {
    if (mouseDate.state <= MouseState.Move) {
      ws.send(JSON.stringify({
        pos_type: MouseState.Click,
        s: 0,
        x: 0,
        y: 0
      }));
    } else if (mouseDate.state === MouseState.Scroll) {
      ws.send(JSON.stringify({
        pos_type: MouseState.RightClick,
        s: 0,
        x: 0,
        y: 0
      }));
    }
  }

  mouseDate.state = MouseState.None;
  mouseDate.point = { x: 0, y: 0 };
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
