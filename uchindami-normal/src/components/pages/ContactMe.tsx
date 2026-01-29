import { useState, useEffect, type FormEvent, type ChangeEvent } from "react";
import { motion, AnimatePresence } from "framer-motion";
import { Send, Mail, MapPin, Loader2, CheckCircle2, Clock, Copy } from "lucide-react";
import emailjs from "@emailjs/browser";
import { toast } from "sonner";
import type { ContactFormData } from "@/lib/types";

// EmailJS configuration
const EMAILJS_SERVICE_ID = import.meta.env.PUBLIC_EMAILJS_SERVICE_ID;
const EMAILJS_TEMPLATE_ID = import.meta.env.PUBLIC_EMAILJS_TEMPLATE_ID;
const EMAILJS_PUBLIC_KEY = import.meta.env.PUBLIC_EMAILJS_PUBLIC_KEY;

const initialFormState: ContactFormData = {
    name: "",
    email: "",
    project: "",
    message: "",
};

export default function ContactMe() {
    const [isSending, setIsSending] = useState(false);
    const [isSent, setIsSent] = useState(false);
    const [form, setForm] = useState<ContactFormData>(initialFormState);
    const [errors, setErrors] = useState<Partial<ContactFormData>>({});
    const [currentTime, setCurrentTime] = useState("");
    const [isCopied, setIsCopied] = useState(false);
    const emailAddress = "manfredchirambojz@gmail.com";

    const handleCopyEmail = async () => {
        try {
            await navigator.clipboard.writeText(emailAddress);
            setIsCopied(true);
            toast.success("Email copied to clipboard!", {
                icon: <CheckCircle2 className="w-4 h-4 text-green-500" />
            });
            setTimeout(() => setIsCopied(false), 2000);
        } catch (err) {
            toast.error("Failed to copy email");
        }
    };

    // Update Malawi Time
    useEffect(() => {
        const updateTime = () => {
            const malawiTime = new Intl.DateTimeFormat("en-US", {
                timeZone: "Africa/Blantyre",
                hour: "2-digit",
                minute: "2-digit",
                hour12: true,
            }).format(new Date());
            setCurrentTime(malawiTime);
        };
        updateTime();
        const timer = setInterval(updateTime, 60000);
        return () => clearInterval(timer);
    }, []);

    const validateField = (name: string, value: string) => {
        if (!value.trim()) return "Required";
        if (name === "email" && !/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(value)) return "Invalid Email";
        return "";
    };

    const handleInputChange = (e: ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) => {
        const { name, value } = e.target;
        setForm(prev => ({ ...prev, [name]: value }));
        setErrors(prev => ({ ...prev, [name]: validateField(name, value) }));
    };

    const handleSubmit = async (e: FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        const newErrors: Partial<ContactFormData> = {};
        let isValid = true;

        (Object.keys(form) as Array<keyof ContactFormData>).forEach(key => {
            const error = validateField(key, form[key]);
            if (error) { newErrors[key] = error; isValid = false; }
        });

        if (!isValid) {
            setErrors(newErrors);
            toast.error("Please check the form fields");
            return;
        }

        setIsSending(true);
        try {
            await emailjs.send(EMAILJS_SERVICE_ID, EMAILJS_TEMPLATE_ID, {
                Name: form.name,
                Email: form.email,
                Project: form.project,
                Message: form.message,
            }, EMAILJS_PUBLIC_KEY);

            setIsSent(true);
            toast.success("Catch you soon!");
            setTimeout(() => {
                setIsSent(false);
                setForm(initialFormState);
            }, 3000);
        } catch (error) {
            toast.error("Failed to send message");
        } finally {
            setIsSending(false);
        }
    };

    return (
        <section>
            <h2 className="section__title">Contact Me</h2>
            <span className="section__subtitle">Get in Touch</span>

            <div className="container">

                <div className="grid grid-cols-1 md:grid-cols-4 md:grid-rows-2 gap-4 h-full">
                    {/* Main Form - Spans 3 columns and all rows on desktop */}
                    <div className="md:col-span-3 md:row-span-2 group">
                        <div className="h-full p-8 rounded-[2rem] bg-surface/30 border border-primary/5 hover:border-primary/20 transition-all duration-500 shadow-sm hover:shadow-xl hover:-translate-y-1 overflow-hidden relative">
                            <form onSubmit={handleSubmit} className="space-y-6 relative z-10">
                                <div className="grid grid-cols-1 sm:grid-cols-2 gap-6">
                                    <BentoInput
                                        label="Name"
                                        name="name"
                                        value={form.name}
                                        error={errors.name}
                                        onChange={handleInputChange}
                                        placeholder="Your name"
                                    />
                                    <BentoInput
                                        label="Email"
                                        name="email"
                                        type="email"
                                        value={form.email}
                                        error={errors.email}
                                        onChange={handleInputChange}
                                        placeholder="your@email.com"
                                    />
                                </div>
                                <BentoInput
                                    label="Subject"
                                    name="project"
                                    value={form.project}
                                    error={errors.project}
                                    onChange={handleInputChange}
                                    placeholder="What's this about?"
                                />
                                <div className="space-y-2">
                                    <label className="text-[10px] font-black uppercase tracking-widest text-primary ml-1">Message</label>
                                    <textarea
                                        name="message"
                                        value={form.message}
                                        onChange={handleInputChange}
                                        rows={4}
                                        className="w-full p-4 bg-background/50 border border-transparent focus:border-primary/20 rounded-2xl outline-none transition-all resize-none text-sm placeholder:text-primary/40"
                                        placeholder="How can I help you?"
                                    />
                                    {errors.message && <span className="text-[10px] text-red-500 font-bold ml-1 uppercase">{errors.message}</span>}
                                </div>

                                <button
                                    type="submit"
                                    disabled={isSending || isSent}
                                    className="w-full py-4 px-8 rounded-2xl bg-primary text-background font-bold text-sm tracking-widest uppercase hover:bg-secondary active:scale-[0.98] transition-all flex items-center justify-center gap-3 group/btn"
                                >
                                    <AnimatePresence mode="wait">
                                        {isSending ? (
                                            <motion.div key="loading" initial={{ opacity: 0 }} animate={{ opacity: 1 }} exit={{ opacity: 0 }}>
                                                <Loader2 className="animate-spin w-5 h-5" />
                                            </motion.div>
                                        ) : isSent ? (
                                            <motion.div key="sent" initial={{ scale: 0.8 }} animate={{ scale: 1 }}>
                                                <CheckCircle2 className="w-5 h-5 text-green-400" />
                                            </motion.div>
                                        ) : (
                                            <motion.div key="idle" className="flex items-center gap-3" initial={{ opacity: 0 }} animate={{ opacity: 1 }}>
                                                <span>Send Message</span>
                                                <Send size={16} className="group-hover/btn:translate-x-1 group-hover/btn:-translate-y-1 transition-transform" />
                                            </motion.div>
                                        )}
                                    </AnimatePresence>
                                </button>
                            </form>

                            {/* Decorative background glow */}
                            <div className="absolute top-0 right-0 w-64 h-64 bg-primary/5 blur-[100px] rounded-full -translate-y-1/2 translate-x-1/2 group-hover:bg-primary/10 transition-colors duration-700" />
                        </div>
                    </div>

                    {/* Email Copy Card */}
                    <button
                        onClick={handleCopyEmail}
                        className="p-8 rounded-[2rem] bg-surface/30 border border-primary/5 flex flex-col items-center justify-center text-center hover:bg-surface/50 hover:border-primary/20 transition-all duration-500 group relative overflow-hidden active:scale-95 min-h-[160px] cursor-pointer w-full"
                    >
                        <div className="relative z-10 flex flex-col items-center">
                            <div className="font-bold text-lg leading-tight text-primary rounded-2xl flex items-center justify-center mb-4 transition-all duration-500 shadow-xl ">
                                {isCopied ? (
                                    <div key="copied">
                                        <CheckCircle2 size={28} />
                                    </div>
                                ) : (
                                    <div key="mail">
                                        <Mail size={28} />
                                    </div>
                                )}

                            </div>
                            <h3 className="font-bold text-lg leading-tight text-primary">
                                {isCopied ? "Email Copied!" : "Direct Mail"}
                            </h3>
                            <p className="text-[10px] font-black uppercase tracking-[0.2em] text-primary/60 mt-3 truncate max-w-[200px]">
                                {emailAddress}
                            </p>
                        </div>

                        {/* Decorative background glow */}
                        <div className="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-40 h-40 bg-primary/5 rounded-full blur-[60px] group-hover:bg-primary/10 transition-colors duration-700" />

                        <div className="transition-all color-primary">
                            <span className="text-[9px] font-black uppercase tracking-[0.2em] text-primary/60">press to copy</span>
                        </div>
                    </button>

                    {/* Location & Time Grid - Spans 2 rows on desktop */}
                    <div className="md:row-start-2 md:col-start-4 grid grid-cols-2 md:grid-cols-1 gap-4">
                        {/* Location Card */}
                        <div className="p-6 rounded-[2rem] bg-surface/30 border border-primary/5 flex flex-col justify-center items-center text-center group hover:bg-surface/50 transition-all">
                            <MapPin size={20} className="text-primary mb-2 group-hover:scale-110 transition-transform" />
                            <h4 className="text-[10px] font-bold uppercase tracking-widest text-primary/60">Location</h4>
                            <span className="text-sm font-bold">Malawi</span>
                        </div>
                        {/* Time Card */}
                        <div className="p-6 rounded-[2rem] bg-surface/30 border border-primary/5 flex flex-col justify-center items-center text-center group hover:bg-surface/50 transition-all">
                            <Clock size={20} className="text-primary mb-2 animate-pulse" />
                            <h4 className="text-[10px] font-bold uppercase tracking-widest text-primary/60">Malawi Time</h4>
                            <span className="text-sm font-bold tabular-nums">{currentTime}</span>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    );
}

function BentoInput({ label, error, ...props }: any) {
    return (
        <div className="space-y-2 w-full">
            <div className="flex justify-between items-center px-1">
                <label className="text-[10px] font-black uppercase tracking-widest text-primary">{label}</label>
                {error && <span className="text-[9px] text-red-500 font-bold uppercase">{error}</span>}
            </div>
            <input
                {...props}
                className="w-full p-4 bg-background/50 border border-transparent focus:border-primary/20 rounded-2xl outline-none transition-all text-sm placeholder:text-primary/40"
            />
        </div>
    );
}
