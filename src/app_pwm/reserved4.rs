#[doc = "Register `Reserved4` reader"]
pub type R = crate::R<Reserved4Spec>;
#[doc = "Register `Reserved4` writer"]
pub type W = crate::W<Reserved4Spec>;
impl W {}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reserved4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reserved4Spec;
impl crate::RegisterSpec for Reserved4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reserved4::R`](R) reader structure"]
impl crate::Readable for Reserved4Spec {}
#[doc = "`write(|w| ..)` method takes [`reserved4::W`](W) writer structure"]
impl crate::Writable for Reserved4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets Reserved4 to value 0"]
impl crate::Resettable for Reserved4Spec {
    const RESET_VALUE: u32 = 0;
}