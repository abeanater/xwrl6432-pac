#[doc = "Register `RTICAFRC0` reader"]
pub type R = crate::R<Rticafrc0Spec>;
#[doc = "Register `RTICAFRC0` writer"]
pub type W = crate::W<Rticafrc0Spec>;
#[doc = "Field `CAFRC0` reader - 31:0\\]
CAFRC0: Capture Free Running Counter 0. This registers captures the current value of the Free Running Counter 0 when a event occurs, controlled by the external capture control block. User and privilege mode (read): value of Free Running Counter 0 on a capture event"]
pub type Cafrc0R = crate::FieldReader<u32>;
#[doc = "Field `CAFRC0` writer - 31:0\\]
CAFRC0: Capture Free Running Counter 0. This registers captures the current value of the Free Running Counter 0 when a event occurs, controlled by the external capture control block. User and privilege mode (read): value of Free Running Counter 0 on a capture event"]
pub type Cafrc0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
CAFRC0: Capture Free Running Counter 0. This registers captures the current value of the Free Running Counter 0 when a event occurs, controlled by the external capture control block. User and privilege mode (read): value of Free Running Counter 0 on a capture event"]
    #[inline(always)]
    pub fn cafrc0(&self) -> Cafrc0R {
        Cafrc0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
CAFRC0: Capture Free Running Counter 0. This registers captures the current value of the Free Running Counter 0 when a event occurs, controlled by the external capture control block. User and privilege mode (read): value of Free Running Counter 0 on a capture event"]
    #[inline(always)]
    #[must_use]
    pub fn cafrc0(&mut self) -> Cafrc0W<Rticafrc0Spec> {
        Cafrc0W::new(self, 0)
    }
}
#[doc = "Capture Free Running Counter 0 current value of free running counter 0 on external event\n\nYou can [`read`](crate::Reg::read) this register and get [`rticafrc0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rticafrc0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rticafrc0Spec;
impl crate::RegisterSpec for Rticafrc0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rticafrc0::R`](R) reader structure"]
impl crate::Readable for Rticafrc0Spec {}
#[doc = "`write(|w| ..)` method takes [`rticafrc0::W`](W) writer structure"]
impl crate::Writable for Rticafrc0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTICAFRC0 to value 0"]
impl crate::Resettable for Rticafrc0Spec {
    const RESET_VALUE: u32 = 0;
}
