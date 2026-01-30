import { useEffect, useState, useCallback } from "react";
import { cn } from "@/lib/utils";
import {
  PiHouseDuotone,
  PiBookOpenDuotone,
  PiUserDuotone,
  PiBriefcaseDuotone,
  PiChatCircleTextDuotone,
} from "react-icons/pi";
import { motion, AnimatePresence } from "framer-motion";
import type { NavItem } from "@/lib/types";
import ThemeToggle from "./ThemeToggle";

interface FloatingNavBarProps {
  variant?: "home" | "blog";
}

const navigation: NavItem[] = [
  { name: "Home", link: "/web/", icon: PiHouseDuotone },
  { name: "Blog", link: "/web/blog", icon: PiBookOpenDuotone },
];

export default function FloatingNavBar({
  variant = "home",
}: FloatingNavBarProps) {
  const [isVisible, setIsVisible] = useState(true);
  const [lastScrollY, setLastScrollY] = useState(0);
  const [activeItem, setActiveItem] = useState("Home");

  const handleScroll = useCallback(() => {
    const currentScrollY = window.scrollY;
    const delta = currentScrollY - lastScrollY;

    // Threshold for hiding: user must have scrolled down at least 200px from top
    // and the specific scroll down motion should be significant or persistent
    if (currentScrollY < 100) {
      setIsVisible(true);
    } else if (delta > 0 && currentScrollY > 200) {
      // Scrolling down and past threshold
      setIsVisible(false);
    } else if (delta < -10) {
      // Scrolling up (with a small buffer to prevent jitter)
      setIsVisible(true);
    }

    setLastScrollY(currentScrollY);
  }, [lastScrollY]);

  useEffect(() => {
    window.addEventListener("scroll", handleScroll, { passive: true });
    return () => window.removeEventListener("scroll", handleScroll);
  }, [handleScroll]);

  return (
    <AnimatePresence>
      <motion.nav
        initial={{ y: 100, x: "-50%", opacity: 0 }}
        animate={{
          y: isVisible ? 0 : 70, // Peek out by ~20-30% instead of hiding completely
          x: "-50%",
          opacity: isVisible ? 1 : 0.4, // Keep it slightly transparent when peeking
        }}
        exit={{ y: 100, x: "-50%", opacity: 0 }}
        transition={{
          duration: 0.8, // Slower, more lethargic animation
          ease: [0.22, 1, 0.36, 1], // Custom quint ease for smoothness
        }}
        className={cn(
          "fixed bottom-8 left-1/2 z-50",
          "px-4 py-3 rounded-full",
          "bg-primary/70 border border-primary/10 shadow-2xl",
          "flex items-center gap-2",
          variant === "blog" && "bg-blog-nav_bg/30 border-blog-nav_hover/20",
        )}
        style={{
          boxShadow: "0 8px 32px 0 rgba(0, 0, 0, 0.37)",
          backdropFilter: "blur(20px)",
          WebkitBackdropFilter: "blur(20px)",
        }}
        aria-label="Main navigation"
      >
        <ul className="flex items-center justify-center gap-1 md:w-[300px]">
          {navigation.map((item) => {
            const Icon = item.icon as any;
            const isActive = activeItem === item.name;

            return (
              <li key={item.name}>
                <a
                  href={item.link}
                  onClick={() => setActiveItem(item.name)}
                  className={cn(
                    "relative flex items-center justify-center p-2.5 rounded-full transition-all duration-300 group",
                    isActive
                      ? "text-background"
                      : "text-background hover:text-background/70",
                  )}
                  aria-label={item.name}
                >
                  {isActive && (
                    <motion.div
                      layoutId="nav-bg"
                      className="absolute inset-0 bg-background/10 rounded-full"
                      transition={{
                        type: "spring",
                        bounce: 0.3,
                        duration: 0.6,
                      }}
                    />
                  )}
                  <Icon
                    className={cn(
                      "w-6 h-6 relative z-10 transition-transform duration-300",
                      isActive ? "scale-110" : "group-hover:scale-110",
                    )}
                  />
                </a>
              </li>
            );
          })}

          <div className="w-[1px] h-6 bg-background mx-1" />

          <li className="flex items-center justify-center p-1">
            <ThemeToggle
              className="text-background hover:text-background/70 transition-colors"
              variant={variant}
            />
          </li>
        </ul>
      </motion.nav>
    </AnimatePresence>
  );
}
