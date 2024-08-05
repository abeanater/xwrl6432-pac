#[doc = "Register `RTICAUC1` reader"]
pub type R = crate::R<Rticauc1Spec>;
#[doc = "Register `RTICAUC1` writer"]
pub type W = crate::W<Rticauc1Spec>;
#[doc = "Field `CAUC1` reader - 31:0\\]
CAUC1: Capture Up Counter 1. This registers captures the current value of the Up Counter 1 when a event occurs, controlled by the external capture control block. The read sequence has to be the same as with Up Counter 1 and Free Running Counter 1. So the RTICAFRC1 register has to be read first, before the RTICAUC1 register is read. This sequence ensures that the value of the RTICAUC1 register is the corresponding value to the RTICAFRC1 register, even if another capture event happens in between the two reads. User and privilege mode (read): value of Up Counter 1 on a capture event"]
pub type Cauc1R = crate::FieldReader<u32>;
#[doc = "Field `CAUC1` writer - 31:0\\]
CAUC1: Capture Up Counter 1. This registers captures the current value of the Up Counter 1 when a event occurs, controlled by the external capture control block. The read sequence has to be the same as with Up Counter 1 and Free Running Counter 1. So the RTICAFRC1 register has to be read first, before the RTICAUC1 register is read. This sequence ensures that the value of the RTICAUC1 register is the corresponding value to the RTICAFRC1 register, even if another capture event happens in between the two reads. User and privilege mode (read): value of Up Counter 1 on a capture event"]
pub type Cauc1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
CAUC1: Capture Up Counter 1. This registers captures the current value of the Up Counter 1 when a event occurs, controlled by the external capture control block. The read sequence has to be the same as with Up Counter 1 and Free Running Counter 1. So the RTICAFRC1 register has to be read first, before the RTICAUC1 register is read. This sequence ensures that the value of the RTICAUC1 register is the corresponding value to the RTICAFRC1 register, even if another capture event happens in between the two reads. User and privilege mode (read): value of Up Counter 1 on a capture event"]
    #[inline(always)]
    pub fn cauc1(&self) -> Cauc1R {
        Cauc1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
CAUC1: Capture Up Counter 1. This registers captures the current value of the Up Counter 1 when a event occurs, controlled by the external capture control block. The read sequence has to be the same as with Up Counter 1 and Free Running Counter 1. So the RTICAFRC1 register has to be read first, before the RTICAUC1 register is read. This sequence ensures that the value of the RTICAUC1 register is the corresponding value to the RTICAFRC1 register, even if another capture event happens in between the two reads. User and privilege mode (read): value of Up Counter 1 on a capture event"]
    #[inline(always)]
    #[must_use]
    pub fn cauc1(&mut self) -> Cauc1W<Rticauc1Spec> {
        Cauc1W::new(self, 0)
    }
}
#[doc = "Capture Up Counter 1 current value of prescale counter 1 on external event\n\nYou can [`read`](crate::Reg::read) this register and get [`rticauc1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rticauc1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rticauc1Spec;
impl crate::RegisterSpec for Rticauc1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rticauc1::R`](R) reader structure"]
impl crate::Readable for Rticauc1Spec {}
#[doc = "`write(|w| ..)` method takes [`rticauc1::W`](W) writer structure"]
impl crate::Writable for Rticauc1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTICAUC1 to value 0"]
impl crate::Resettable for Rticauc1Spec {
    const RESET_VALUE: u32 = 0;
}
