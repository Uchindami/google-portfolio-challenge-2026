import * as React from "react";
import type { SVGProps } from "react";

const BlogIcon = (props: SVGProps<SVGSVGElement>) => (
  <svg
    xmlns="http://www.w3.org/2000/svg"
    viewBox="0 0 320 512"
    width="1em"
    height="1em"
    {...props}
  >
    <path d="M64 32C28.7 32 0 60.7 0 96v320c0 35.3 28.7 64 64 64h128c70.7 0 128-57.3 128-128 0-46.5-24.8-87.3-62-109.7 18.7-22.3 30-51 30-82.3 0-70.7-57.3-128-128-128H64zm96 192H64V96h96c35.3 0 64 28.7 64 64s-28.7 64-64 64zm-96 64h128c35.3 0 64 28.7 64 64s-28.7 64-64 64H64V288z" />
  </svg>
);
export default BlogIcon;
