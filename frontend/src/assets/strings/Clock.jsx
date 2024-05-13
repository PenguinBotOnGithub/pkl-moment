import { useState } from "react";

const Clock = () => {
  // For digital clock
  let [ctime, setCTime] = useState(new Date().toLocaleTimeString());
  const updateTime = () => {
    setCTime(new Date().toLocaleTimeString());
  };
  setInterval(updateTime, 1000);
  return (ctime);
};
export default Clock;
