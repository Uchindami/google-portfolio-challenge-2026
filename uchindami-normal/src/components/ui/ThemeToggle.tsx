import { useEffect, useState } from "react";
import { Sun, Moon } from "lucide-react";
import { cn } from "@/lib/utils";

interface ThemeToggleProps {
    className?: string;
    variant?: "home" | "blog";
}

export default function ThemeToggle({ className, variant = "home" }: ThemeToggleProps) {
    const [theme, setTheme] = useState<"light" | "dark" | null>(null);

    useEffect(() => {
        // Initialize theme from document or localStorage
        const savedTheme = localStorage.getItem("theme") as "light" | "dark" | null;
        const initialTheme = savedTheme || (document.documentElement.classList.contains("dark") ? "dark" : "light");
        setTheme(initialTheme);
    }, []);

    const toggleTheme = () => {
        const newTheme = theme === "dark" ? "light" : "dark";
        setTheme(newTheme);

        if (newTheme === "dark") {
            document.documentElement.classList.add("dark");
        } else {
            document.documentElement.classList.remove("dark");
        }

        localStorage.setItem("theme", newTheme);
    };

    // Prevent hydration mismatch by not rendering icons until client-side mount
    if (theme === null) {
        return <div className={cn("w-10 h-10", className)} />;
    }

    const isDark = theme === "dark";

    return (
        <button
            onClick={toggleTheme}
            className={cn(
                "flex items-center justify-center",
                "w-10 h-10 rounded-full",
                "transition-all duration-300 ease-in-out",
                "hover:scale-110 active:scale-95",
                className
            )}
            aria-label={`Switch to ${isDark ? "light" : "dark"} theme`}
        >
            {isDark ? (
                <Sun className="w-6 h-6 animate-in fade-in zoom-in spin-in duration-500" />
            ) : (
                <Moon className="w-6 h-6 animate-in fade-in zoom-in spin-in-180 duration-500" />
            )}
        </button>
    );
}
