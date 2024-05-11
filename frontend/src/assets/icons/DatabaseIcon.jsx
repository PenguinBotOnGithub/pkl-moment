import React from 'react';

export const DatabaseIcon = ({size, stroke}) => {
  return (
    <svg xmlns="http://www.w3.org/2000/svg" width={size} height={size} fill="none" viewBox="0 0 192 192">
  <ellipse cx="96" cy="47" stroke={stroke} stroke-width="12" rx="55" ry="25"/>
  <path stroke={stroke} stroke-width="12" d="M151 80c0 13.807-24.624 25-55 25S41 93.807 41 80m110 32c0 13.807-24.624 25-55 25s-55-11.193-55-25m110 33c0 13.807-24.624 25-55 25s-55-11.193-55-25m0-98v98m110-98v98"/>
</svg>

  );
};