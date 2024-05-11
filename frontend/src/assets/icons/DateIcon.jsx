import React from 'react';

export const DateIcon = ({size, stroke}) => {
  return (
    <svg xmlns="http://www.w3.org/2000/svg" width={size} height={size} fill="none" viewBox="0 0 192 192">
  <path stroke={stroke} stroke-linejoin="round" stroke-width="12" d="M164 46H28a6 6 0 0 0-6 6v101a6 6 0 0 0 6 6h136a6 6 0 0 0 6-6V52a6 6 0 0 0-6-6ZM22 86h148"/>
  <path stroke={stroke} stroke-linecap="round" stroke-linejoin="round" stroke-width="12" d="M63 31v30m66-30v30"/>
  <circle cx="46" cy="135" r="3" stroke={stroke} stroke-width="6"/>
  <circle cx="46" cy="110" r="3" stroke={stroke} stroke-width="6"/>
  <circle cx="71" cy="110" r="3" stroke={stroke} stroke-width="6"/>
  <circle cx="71" cy="135" r="3" stroke={stroke} stroke-width="6"/>
  <circle cx="96" cy="110" r="3" stroke={stroke} stroke-width="6"/>
  <circle cx="96" cy="135" r="3" stroke={stroke} stroke-width="6"/>
  <circle cx="121" cy="110" r="3" stroke={stroke} stroke-width="6"/>
  <circle cx="121" cy="135" r="3" stroke={stroke} stroke-width="6"/>
  <circle cx="146" cy="110" r="3" stroke={stroke} stroke-width="6"/>
</svg>

  );
};