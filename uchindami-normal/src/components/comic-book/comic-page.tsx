import { ParallaxLayer } from "@react-spring/parallax";
import { motion } from "framer-motion";
import React from "react";

export const ComicPage = ({ background, character, text, pageIndex }) => (
  <>
    <ParallaxLayer
      offset={pageIndex}
      speed={0}
      factor={1}
      className="border-8 p-0 m-0 border-black"
    >
      <div
        className="w-full h-full bg-cover bg-center"
        style={{ backgroundImage: `url(${background})` }}
      />
    </ParallaxLayer>
    <ParallaxLayer offset={pageIndex} speed={0.4}>
      <div className="flex justify-center items-center h-full">
        <motion.img
          src={character}
          alt="Comic character"
          className="w-1/2 h-auto"
          initial={{ scale: 0.9, rotate: -5 }}
          animate={{ scale: 1, rotate: 5 }}
          transition={{ duration: 2, repeat: Infinity, repeatType: "reverse" }}
        />
      </div>
    </ParallaxLayer>
    <ParallaxLayer offset={pageIndex} speed={0.7}>
      <div className="flex justify-center items-center h-full">
        <motion.div
          className="bg-white border-4 border-black rounded-lg p-4 w-1/2 transform rotate-3 shadow-lg"
          style={{ fontFamily: "Comic Sans MS, cursive" }}
          initial={{ opacity: 0, y: 50 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.5, duration: 1 }}
        >
          <p className="text-2xl font-bold text-center">{text}</p>
        </motion.div>
      </div>
    </ParallaxLayer>
  </>
);
