#[doc = "Register `dft_proc_dmled_status` reader"]
pub type R = crate::R<DftProcDmledStatusSpec>;
#[doc = "Register `dft_proc_dmled_status` writer"]
pub type W = crate::W<DftProcDmledStatusSpec>;
impl W {}
#[doc = "dft_proc_dmled_status\n\nYou can [`read`](crate::Reg::read) this register and get [`dft_proc_dmled_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dft_proc_dmled_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DftProcDmledStatusSpec;
impl crate::RegisterSpec for DftProcDmledStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dft_proc_dmled_status::R`](R) reader structure"]
impl crate::Readable for DftProcDmledStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`dft_proc_dmled_status::W`](W) writer structure"]
impl crate::Writable for DftProcDmledStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets dft_proc_dmled_status to value 0"]
impl crate::Resettable for DftProcDmledStatusSpec {
    const RESET_VALUE: u32 = 0;
}
