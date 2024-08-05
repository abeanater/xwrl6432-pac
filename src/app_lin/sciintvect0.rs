#[doc = "Register `SCIINTVECT0` reader"]
pub type R = crate::R<Sciintvect0Spec>;
#[doc = "Register `SCIINTVECT0` writer"]
pub type W = crate::W<Sciintvect0Spec>;
#[doc = "Field `INTVECT0` reader - 4:0\\]
Interrupt vector offset for INT0. This register indicates the offset for interrupt line INT0. A read to this register updates its value to the next highest priority pending interrupt in SCIFLR and clears the flag corresponding to the offset that was read. Note: The flags for the receive (SCIFLR.9) and the transmit (SCIFLR.8) interrupts cannot be cleared by reading the corresponding offset vector in this register (see detailed description in SCIFLR register)."]
pub type Intvect0R = crate::FieldReader;
#[doc = "Field `INTVECT0` writer - 4:0\\]
Interrupt vector offset for INT0. This register indicates the offset for interrupt line INT0. A read to this register updates its value to the next highest priority pending interrupt in SCIFLR and clears the flag corresponding to the offset that was read. Note: The flags for the receive (SCIFLR.9) and the transmit (SCIFLR.8) interrupts cannot be cleared by reading the corresponding offset vector in this register (see detailed description in SCIFLR register)."]
pub type Intvect0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `Reserved1` reader - 31:16\\]
Reserved"]
pub type Reserved1R = crate::FieldReader<u16>;
#[doc = "Field `Reserved1` writer - 31:16\\]
Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt vector offset for INT0. This register indicates the offset for interrupt line INT0. A read to this register updates its value to the next highest priority pending interrupt in SCIFLR and clears the flag corresponding to the offset that was read. Note: The flags for the receive (SCIFLR.9) and the transmit (SCIFLR.8) interrupts cannot be cleared by reading the corresponding offset vector in this register (see detailed description in SCIFLR register)."]
    #[inline(always)]
    pub fn intvect0(&self) -> Intvect0R {
        Intvect0R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Interrupt vector offset for INT0. This register indicates the offset for interrupt line INT0. A read to this register updates its value to the next highest priority pending interrupt in SCIFLR and clears the flag corresponding to the offset that was read. Note: The flags for the receive (SCIFLR.9) and the transmit (SCIFLR.8) interrupts cannot be cleared by reading the corresponding offset vector in this register (see detailed description in SCIFLR register)."]
    #[inline(always)]
    #[must_use]
    pub fn intvect0(&mut self) -> Intvect0W<Sciintvect0Spec> {
        Intvect0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<Sciintvect0Spec> {
        Reserved1W::new(self, 16)
    }
}
#[doc = "The SCIINTVECT0 register indicates the offset for the INT0 interrupt line.\n\nYou can [`read`](crate::Reg::read) this register and get [`sciintvect0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sciintvect0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sciintvect0Spec;
impl crate::RegisterSpec for Sciintvect0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sciintvect0::R`](R) reader structure"]
impl crate::Readable for Sciintvect0Spec {}
#[doc = "`write(|w| ..)` method takes [`sciintvect0::W`](W) writer structure"]
impl crate::Writable for Sciintvect0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCIINTVECT0 to value 0"]
impl crate::Resettable for Sciintvect0Spec {
    const RESET_VALUE: u32 = 0;
}
