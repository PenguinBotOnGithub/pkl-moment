import React from 'react';

export const PersonIcon = ({size, stroke}) => {
  return (
    <svg xmlns="http://www.w3.org/2000/svg" width={size} height={size} fill="none" viewBox="0 0 192 192">
  <path stroke={stroke} stroke-width="12" d="M43 158c0 6.627 5.373 12 12 12h82c6.627 0 12-5.373 12-12v-14.429a36.99 36.99 0 0 0-10.879-26.15 37.194 37.194 0 0 0-26.221-10.85H80.1a37.195 37.195 0 0 0-26.221 10.85A36.99 36.99 0 0 0 43 143.571V158Z"/>
  <circle cx="96" cy="56" r="34" stroke={stroke} stroke-width="12"/>
</svg>

  );
};