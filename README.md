# RUSTY ROBOT

<style>
  
.body {
  background-color: white;
  width: 100vw;
  height: 100vh;
  display: flex;
  justify-content: center;
  align-items: center;
  overflow: hidden;
}

.cube-container {
  width: 180px;
  height: 180px;
  perspective: 45em;
}

.cube {
  width: 100%;
  height: 100%;
  position: relative;
  transform-style: preserve-3d;
  transition-duration: 2s;
  transform: rotateX(-15deg) rotateY(20deg);
  animation-name: flipEl;
  animation-duration: 6s;
  animation-iteration-count: infinite;
  animation-timing-function: linear;
}

.cube-side {
  position: absolute;
  width: 100%;
  height: 100%;
  background-color: #988EF3;
  border: 1px solid black;
}

.cube-side:nth-child(1) {
  transform: rotateY(0deg) translateZ(90px);
}
.cube-side:nth-child(2) {
  transform: rotateY(90deg) translateZ(90px);

}
.cube-side:nth-child(3) {
  transform: rotateY(180deg) translateZ(90px);

}
.cube-side:nth-child(4) {
  transform: rotateY(-90deg) translateZ(90px);

}

@keyframes flipEl {
  from {
    transform: rotateY(0deg);
  }

  to {
    transform: rotateY(360deg);
  }
}

</style>
<div class="body">
<div class="cube-container">
  <div class="cube">
    <div class="cube-side">
      <svg viewBox="0 0 120 120">
        <circle fill="#4671BB" cx="30" cy="30" r="14"/>
        <circle fill="#A1BCD9" cx="30" cy="30" r="6"/>
        <circle fill="#4671BB" cx="90" cy="30" r="14"/>
        <circle fill="#A1BCD9" cx="90" cy="30" r="6"/>
      </svg>
    </div>
    <div class="cube-side"></div>
    <div class="cube-side"></div>
    <div class="cube-side"></div>
  </div>
</div>
</div>