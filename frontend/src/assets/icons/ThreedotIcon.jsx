import React from 'react';

export const ThreedotIcon = ({size, stroke}) => {
  return (
    <svg xmlns="http://www.w3.org/2000/svg" width={size} height={size} fill="none" viewBox="0 0 192 192">
  <circle cx="96" cy="38" r="16" stroke={stroke} stroke-linejoin="round" stroke-width="12"/>
  <circle cx="96.5" cy="153.5" r="16.5" stroke={stroke} stroke-linejoin="round" stroke-width="12"/>
  <circle cx="96" cy="96" r="16" stroke={stroke} stroke-linejoin="round" stroke-width="12"/>
</svg>

  );
};