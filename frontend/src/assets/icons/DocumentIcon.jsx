import React from 'react';

export const DocumentIcon = ({size, stroke}) => {
  return (
    <svg xmlns="http://www.w3.org/2000/svg" width={size} height={size} fill="none" viewBox="0 0 192 192">
  <path stroke={stroke} stroke-linejoin="round" stroke-width="12" d="M144 61h-34a3 3 0 0 1-3-3V24m39 38.485V164a6 6 0 0 1-6 6H52a6 6 0 0 1-6-6V28a6 6 0 0 1 6-6h53.515a6 6 0 0 1 4.242 1.757l34.486 34.486A6 6 0 0 1 146 62.485Z"/>
  <path stroke={stroke} stroke-linecap="round" stroke-linejoin="round" stroke-width="12" d="M66 95h60m-60 25h20m20 0h20M66 70h15m-15 75h40"/>
</svg>

  );
};