/**
 * Portfolio Project Data
 * Centralized data for all portfolio projects
 */
import type { Project } from "@/lib/types";

// Import images
import Uchi from "@/assets/uchi.png";
import Genrescope from "@/assets/genrescope.png";
import Mazuma from "@/assets/Mazuma.png";
import Ndipatse from "@/assets/lift.png";
import EasyApply from "@/assets/easyapply.png";
import MalawiJobs from "@/assets/malawiAPI.png";
import Apply from "@/assets/applyMalawi.png";

export const projects: Project[] = [
  {
    id: 1,
    title: "Apply Malawi",
    description: `My latest project, a cross platform cv maker. Find it on whatsapp or web`,
    image: Apply,
    tags: ["Go", "Typst", "React", "Docker", "NGINX",],
    links: {
      github: "#",
      live: "https://applymalawi.com",
    },
    featured: true,
  },
  {
    id: 2,
    title: "Uchindami",
    description: `My first portfolio, before the ai age`,
    image: Uchi,
    tags: ["Astro", "HTML", "CSS", "JavaScript"],
    links: {
      github: "https://github.com/Uchindami/astronautUchindami",
      live: "https://uchindami.xyz/",
    },
    featured: false,
  },
  {
    id: 3,
    title: "Mazuma Desktop",
    description: `OOP at its peak! 
    Java Java Java ........
    A desktop app that allowed users to search for cheaper prices for products. Built using Java and JavaFX - a great learning experience.`,
    image: Mazuma,
    tags: ["Java", "JavaFX", "OOP"],
    links: {
      github: "https://github.com/Uchindami/Mazuma_Web-Crawler",
      live: "#",
    },
  },
  {
    id: 3,
    title: "Genrescope",
    description: `My entry into the world of APIs and AI. Genrescope scans your Spotify listening history and tries to describe your personality. First project I shared publicly with great feedback!`,
    image: Genrescope,
    tags: ["React", "Vite", "Tailwind CSS", "OpenAI API", "Spotify API"],
    links: {
      github: "https://github.com/Uchindami/genrescope",
      live: "https://genrescope.uchindami.xyz",
    },
  },
  {
    id: 4,
    title: "Ndipatse Lift",
    description: `My first published mobile app! A ride-sharing app connecting drivers and passengers in Malawi. Check it out on the Play Store.`,
    image: Ndipatse,
    tags: ["React Native", "Tailwind CSS", "Maps"],
    links: {
      github: "https://github.com/Uchindami/ndipatse-lift",
      live: "https://play.google.com/store/apps?hl=en&pli=1",
    },
  },
  {
    id: 5,
    title: "Malawi Jobs API",
    description: `An API that allows users to search for jobs in Malawi. Built with a friend using Node.js and Go - a collaborative learning experience.`,
    image: MalawiJobs,
    tags: ["Go", "Node.js", "REST API"],
    links: {
      github: "https://github.com/Uchindami/malawiJobsApi",
      live: "#",
    },
  },
  {
    id: 6,
    title: "EasyApply",
    description: `This project has been sunset in favor of Apply Malawi`,
    image: EasyApply,
    tags: ["Go", "GraphQL", "React Router", "AI"],
    links: {
      github: "https://github.com/Uchindami/easy-apply",
      live: "https://applymalawi.com",
    },
  },
];
