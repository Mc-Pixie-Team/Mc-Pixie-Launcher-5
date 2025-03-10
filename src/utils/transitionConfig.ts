import gsap from 'gsap';





const pageTransition = {
  name: 'page-transiton',
  mode: 'out-in',
  onEnter: (el: any, done: any) => {
    console.log("ENTER")
    gsap.set(el, { autoAlpha: 0, scale: 0.9, xPercent: 100 });
    gsap
      .timeline({
        paused: true,
        onComplete() {
          done();
        },
      })
      .to(el, { autoAlpha: 1, xPercent: 0, duration: 0.25})
      .to(el, {scale: 1, duration: 0.4, ease: "expo.out"})
      
      .play();
  },
  onLeave: (el : any, done: any) => {
    console.log("LEAVE")
    gsap
      .timeline({ paused: true, onComplete: () => {
        console.log("LEAVE DONE")
        done();
      } })
      .to(el, { autoAlpha: 1, xPercent: 100, duration: 0.2})
      
      .play();
     
  },
};

export default pageTransition;
