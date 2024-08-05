#[doc = "Register `RTICAUC0` reader"]
pub type R = crate::R<Rticauc0Spec>;
#[doc = "Register `RTICAUC0` writer"]
pub type W = crate::W<Rticauc0Spec>;
#[doc = "Field `CAUC0` reader - 31:0\\]
CAUC0: Capture Up Counter 0. This registers captures the current value of the Up Counter 0 when a event occurs, controlled by the external capture control block. The read sequence has to be the same as with Up Counter 0 and Free Running Counter 0. So the RTICAFRC0 register has to be read first, before the RTICAUC0 register is read. This sequence ensures that the value of the RTICAUC0 register is the corresponding value to the RTICAFRC0 register, even if another capture event happens in between the two reads. User and privilege mode (read): value of Up Counter 0 on a capture event"]
pub type Cauc0R = crate::FieldReader<u32>;
#[doc = "Field `CAUC0` writer - 31:0\\]
CAUC0: Capture Up Counter 0. This registers captures the current value of the Up Counter 0 when a event occurs, controlled by the external capture control block. The read sequence has to be the same as with Up Counter 0 and Free Running Counter 0. So the RTICAFRC0 register has to be read first, before the RTICAUC0 register is read. This sequence ensures that the value of the RTICAUC0 register is the corresponding value to the RTICAFRC0 register, even if another capture event happens in between the two reads. User and privilege mode (read): value of Up Counter 0 on a capture event"]
pub type Cauc0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
CAUC0: Capture Up Counter 0. This registers captures the current value of the Up Counter 0 when a event occurs, controlled by the external capture control block. The read sequence has to be the same as with Up Counter 0 and Free Running Counter 0. So the RTICAFRC0 register has to be read first, before the RTICAUC0 register is read. This sequence ensures that the value of the RTICAUC0 register is the corresponding value to the RTICAFRC0 register, even if another capture event happens in between the two reads. User and privilege mode (read): value of Up Counter 0 on a capture event"]
    #[inline(always)]
    pub fn cauc0(&self) -> Cauc0R {
        Cauc0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
CAUC0: Capture Up Counter 0. This registers captures the current value of the Up Counter 0 when a event occurs, controlled by the external capture control block. The read sequence has to be the same as with Up Counter 0 and Free Running Counter 0. So the RTICAFRC0 register has to be read first, before the RTICAUC0 register is read. This sequence ensures that the value of the RTICAUC0 register is the corresponding value to the RTICAFRC0 register, even if another capture event happens in between the two reads. User and privilege mode (read): value of Up Counter 0 on a capture event"]
    #[inline(always)]
    #[must_use]
    pub fn cauc0(&mut self) -> Cauc0W<Rticauc0Spec> {
        Cauc0W::new(self, 0)
    }
}
#[doc = "Capture Up Counter 0 current value of prescale counter 0 on external event\n\nYou can [`read`](crate::Reg::read) this register and get [`rticauc0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rticauc0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rticauc0Spec;
impl crate::RegisterSpec for Rticauc0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rticauc0::R`](R) reader structure"]
impl crate::Readable for Rticauc0Spec {}
#[doc = "`write(|w| ..)` method takes [`rticauc0::W`](W) writer structure"]
impl crate::Writable for Rticauc0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTICAUC0 to value 0"]
impl crate::Resettable for Rticauc0Spec {
    const RESET_VALUE: u32 = 0;
}
