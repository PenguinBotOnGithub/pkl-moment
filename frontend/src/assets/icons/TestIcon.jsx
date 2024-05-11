import React from 'react';

export const TestIcon = ({size, stroke}) => {
  return (
    <svg xmlns="http://www.w3.org/2000/svg" width={size} height={size} fill="none" viewBox="0 0 192 192">
  <path stroke={stroke} stroke-linecap="round" stroke-linejoin="round" stroke-width="12" d="M42 96h95.411c5.049 0 7.84 5.857 4.66 9.779l-19.007 23.442a6 6 0 0 0 0 7.558l19.007 23.442c3.18 3.922.389 9.779-4.66 9.779H48a6 6 0 0 1-6-6V96Zm71 0H42s17-10.5 17-37-17-37-17-37h71c20.435 0 37 16.566 37 37s-16.565 37-37 37Z"/>
</svg>

  );
};