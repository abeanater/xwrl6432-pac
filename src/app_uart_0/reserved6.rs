#[doc = "Register `RESERVED6` reader"]
pub type R = crate::R<Reserved6Spec>;
#[doc = "Register `RESERVED6` writer"]
pub type W = crate::W<Reserved6Spec>;
impl W {}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reserved6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reserved6Spec;
impl crate::RegisterSpec for Reserved6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reserved6::R`](R) reader structure"]
impl crate::Readable for Reserved6Spec {}
#[doc = "`write(|w| ..)` method takes [`reserved6::W`](W) writer structure"]
impl crate::Writable for Reserved6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RESERVED6 to value 0"]
impl crate::Resettable for Reserved6Spec {
    const RESET_VALUE: u32 = 0;
}
