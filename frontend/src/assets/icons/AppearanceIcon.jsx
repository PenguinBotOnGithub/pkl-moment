import React from 'react';

export const AppearanceIcon = ({size, stroke}) => {
  return (
    <svg xmlns="http://www.w3.org/2000/svg" width={size} height={size} fill="none" viewBox="0 0 192 192">
  <path stroke={stroke} stroke-linejoin="round" stroke-width="12" d="M55 170c18.225 0 33-14.775 33-33s-14.775-33-33-33-33 14.775-33 33v27a6 6 0 0 0 6 6h27Z"/>
  <path stroke={stroke} stroke-linecap="round" stroke-linejoin="round" stroke-width="12" d="M66 103.043s20.106-36.059 39.956-52.565C131.05 29.611 170 22 170 22s-7.611 38.95-28.478 64.044C125.015 105.894 88.957 126 88.957 126"/>
  <path stroke={stroke} stroke-linecap="round" stroke-linejoin="round" stroke-width="12" d="M85 73.828s11.194 5.538 17.678 12.021c6.483 6.484 12.02 17.678 12.02 17.678"/>
</svg>

  );
};