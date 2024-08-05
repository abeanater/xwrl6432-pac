#[doc = "Register `Reserved7` reader"]
pub type R = crate::R<Reserved7Spec>;
#[doc = "Register `Reserved7` writer"]
pub type W = crate::W<Reserved7Spec>;
impl W {}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reserved7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reserved7Spec;
impl crate::RegisterSpec for Reserved7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reserved7::R`](R) reader structure"]
impl crate::Readable for Reserved7Spec {}
#[doc = "`write(|w| ..)` method takes [`reserved7::W`](W) writer structure"]
impl crate::Writable for Reserved7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets Reserved7 to value 0"]
impl crate::Resettable for Reserved7Spec {
    const RESET_VALUE: u32 = 0;
}
