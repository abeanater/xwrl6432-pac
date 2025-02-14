#[doc = "Register `Reserved2` reader"]
pub type R = crate::R<Reserved2Spec>;
#[doc = "Register `Reserved2` writer"]
pub type W = crate::W<Reserved2Spec>;
impl W {}
#[doc = "Reserved2\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reserved2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reserved2Spec;
impl crate::RegisterSpec for Reserved2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reserved2::R`](R) reader structure"]
impl crate::Readable for Reserved2Spec {}
#[doc = "`write(|w| ..)` method takes [`reserved2::W`](W) writer structure"]
impl crate::Writable for Reserved2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets Reserved2 to value 0"]
impl crate::Resettable for Reserved2Spec {
    const RESET_VALUE: u32 = 0;
}
