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
  LeftDown = 3,
  LeftUp = 4,
  RightDown = 5,
  RightUp = 6,
  Click = 7,
  RightClick = 8,
  Drag = 9,
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

  // ws.send('hello 123');
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

}, (newDate, oldDate) => {
  if (newDate.state === MouseState.None ||
    (newDate.point.x === 0 && newDate.point.y === 0 || oldDate.point.x === 0 && oldDate.point.y === 0)) {
    return;
  }

  const newPoint = newDate.point;
  const oldPoint = oldDate.point;
  // console.log('watch', newDate);
  if (newDate.state === MouseState.Move) {
    const deltaPos = { x: newPoint.x - oldPoint.x, y: newPoint.y - oldPoint.y };

    const x = Math.floor(scale(deltaPos.x, 1.5, 1.1));
    const y = Math.floor(scale(deltaPos.y, 1.5, 1.1));
    if (x === 0 && y === 0) {
      return;
    }
    ws.send(JSON.stringify({
      pos_type: newDate.state,
      s: 0,
      x,
      y
    }));
  } else if (newDate.state === MouseState.Scroll) {
    const delta = newPoint.y - oldPoint.y;
    scrollJudgeOffset += delta;

    if (Math.abs(scrollJudgeOffset) > 6) {
      const d = {
        pos_type: newDate.state,
        s: Math.round(scrollJudgeOffset / 6),
        x: 0,
        y: 0
      }
      ws.send(JSON.stringify(d));
      scrollJudgeOffset = 0;
    }
  } else if (newDate.state === MouseState.Drag && oldDate.state === MouseState.Drag) {
    const deltaPos = { x: newPoint.x - oldPoint.x, y: newPoint.y - oldPoint.y };
    const x = Math.floor(scale(deltaPos.x, 1.5, 1.1));
    const y = Math.floor(scale(deltaPos.y, 1.5, 1.1));
    if (x === 0 && y === 0) {
      return;
    }
    ws.send(JSON.stringify({
      pos_type: newDate.state,
      s: 0,
      x,
      y
    }));
  }

});

function onTouch(e: TouchEvent) {
  // console.log('e.touchs', e)
  switch (mouseDate.state) {
    case MouseState.None:
      clickCheckTime = Date.now();
      switch (e.touches.length) {
        case 1:
          mouseDate.point = { x: 0, y: 0 };
          mouseDate.state = MouseState.Move;
          break;
        case 2:
          mouseDate.point = { x: 0, y: 0 };
          mouseDate.state = MouseState.Scroll;
          return;
        case 3:
          mouseDate.point = { x: 0, y: 0 };
          mouseDate.state = MouseState.Drag;
          return;
      }
      handleMouseMove(e.touches);
      break;
    case MouseState.Move:
      switch (e.touches.length) {
        case 1:
          break;
        case 2:
          mouseDate.point = { x: 0, y: 0 };
          mouseDate.state = MouseState.Scroll;
          return;
        case 3:
          mouseDate.point = { x: 0, y: 0 };
          mouseDate.state = MouseState.Drag;
          return;
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
          ws.send(JSON.stringify({
            pos_type: MouseState.LeftDown,
            s: 0,
            x: 0,
            y: 0
          }));
          break;
      }
      handleMouseMove(e.touches);
      break;
    case MouseState.Drag:
      handleMouseMove(e.touches);
      break;
  }
  e.preventDefault();
}
function handleMouseMove(touchList: TouchList) {
  if (touchList.length === 0) {
    return;
  }
  if (touchList.length === 1 && mouseDate.state !== MouseState.Move) {
    return;
  }
  if (touchList.length === 2 && mouseDate.state !== MouseState.Scroll) {
    return;
  }
  if (touchList.length === 3 && mouseDate.state !== MouseState.Drag) {
    return;
  }
  const p0 = touchList[touchList.length - 1];
  mouseDate.point = { x: p0.clientX, y: p0.clientY };
}

function onTouchEnd(e: TouchEvent) {

  if (e.touches.length === 0) {
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

    // check drag end
    if (mouseDate.state === MouseState.Drag) {
      ws.send(JSON.stringify({
        pos_type: MouseState.LeftUp,
        s: 0,
        x: 0,
        y: 0
      }));
    }

    mouseDate.state = MouseState.None;

  }
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
