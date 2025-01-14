#[doc = "Register `Reserved1` reader"]
pub type R = crate::R<Reserved1Spec>;
#[doc = "Register `Reserved1` writer"]
pub type W = crate::W<Reserved1Spec>;
impl W {}
#[doc = "Reserved1\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reserved1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reserved1Spec;
impl crate::RegisterSpec for Reserved1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reserved1::R`](R) reader structure"]
impl crate::Readable for Reserved1Spec {}
#[doc = "`write(|w| ..)` method takes [`reserved1::W`](W) writer structure"]
impl crate::Writable for Reserved1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets Reserved1 to value 0"]
impl crate::Resettable for Reserved1Spec {
    const RESET_VALUE: u32 = 0;
}
