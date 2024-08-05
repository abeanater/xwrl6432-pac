#[doc = "Register `RESERVED9` reader"]
pub type R = crate::R<Reserved9Spec>;
#[doc = "Register `RESERVED9` writer"]
pub type W = crate::W<Reserved9Spec>;
impl W {}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reserved9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reserved9Spec;
impl crate::RegisterSpec for Reserved9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reserved9::R`](R) reader structure"]
impl crate::Readable for Reserved9Spec {}
#[doc = "`write(|w| ..)` method takes [`reserved9::W`](W) writer structure"]
impl crate::Writable for Reserved9Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RESERVED9 to value 0"]
impl crate::Resettable for Reserved9Spec {
    const RESET_VALUE: u32 = 0;
}
