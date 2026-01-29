import { PiRssSimpleDuotone } from "react-icons/pi";
import React from "react";
import { cn } from "@/lib/utils";

export const Navigation = () => {
  const links = [
    { href: "/", text: "Home", isIcon: false },
    { href: "/blog", text: "Blog", isIcon: false },
    { href: "/tags/", text: "Tags", isIcon: false },
    {
      href: "/rss.xml",
      icon: <PiRssSimpleDuotone />,
      ariaLabel: "RSS Feed",
      rel: "noopener noreferrer",
      isIcon: true,
    },
  ];

  return (
    <>
      {links.map((link, index) => (
        <a
          key={index}
          href={link.href}
          className={cn(
            "font-bold text-blog-third text-sm hover:text-blog-secondary",
            link.isIcon ? "text-lg" : "",
          )}
          rel={link.rel}
          aria-label={link.ariaLabel}
        >
          {link.icon || link.text}
        </a>
      ))}
    </>
  );
};
