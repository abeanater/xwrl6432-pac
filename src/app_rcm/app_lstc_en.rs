#[doc = "Register `APP_LSTC_EN` reader"]
pub type R = crate::R<AppLstcEnSpec>;
#[doc = "Register `APP_LSTC_EN` writer"]
pub type W = crate::W<AppLstcEnSpec>;
#[doc = "Field `enable` reader - 0:0\\]
Enable vbusp_req and clk_en for app lstc"]
pub type EnableR = crate::BitReader;
#[doc = "Field `enable` writer - 0:0\\]
Enable vbusp_req and clk_en for app lstc"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enable vbusp_req and clk_en for app lstc"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enable vbusp_req and clk_en for app lstc"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<AppLstcEnSpec> {
        EnableW::new(self, 0)
    }
}
#[doc = "APP_LSTC_EN\n\nYou can [`read`](crate::Reg::read) this register and get [`app_lstc_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_lstc_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppLstcEnSpec;
impl crate::RegisterSpec for AppLstcEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`app_lstc_en::R`](R) reader structure"]
impl crate::Readable for AppLstcEnSpec {}
#[doc = "`write(|w| ..)` method takes [`app_lstc_en::W`](W) writer structure"]
impl crate::Writable for AppLstcEnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APP_LSTC_EN to value 0"]
impl crate::Resettable for AppLstcEnSpec {
    const RESET_VALUE: u32 = 0;
}
