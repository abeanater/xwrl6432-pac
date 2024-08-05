#[doc = "Register `CLR_MISC_CLIP` reader"]
pub type R = crate::R<ClrMiscClipSpec>;
#[doc = "Register `CLR_MISC_CLIP` writer"]
pub type W = crate::W<ClrMiscClipSpec>;
#[doc = "Field `CLR_MISC_CLIP` reader - 0:0\\]
This clears the following clip register :- dc_acc_clip_status dc_est_clip_status intf_stats_mag_accumulator_clip_status Intf_stats_magdiff_accumulator_clip_status intf_stats_thresh_mag_clip_status intf_stats_thresh_magdiff_clip_status ip_formatter_clip_status op_formatter_clip_status intf_stats_sum_mag_val_clip_status intf_stats_sum_magdiff_val_clip_status Its a self clearing bit"]
pub type ClrMiscClipR = crate::BitReader;
#[doc = "Field `CLR_MISC_CLIP` writer - 0:0\\]
This clears the following clip register :- dc_acc_clip_status dc_est_clip_status intf_stats_mag_accumulator_clip_status Intf_stats_magdiff_accumulator_clip_status intf_stats_thresh_mag_clip_status intf_stats_thresh_magdiff_clip_status ip_formatter_clip_status op_formatter_clip_status intf_stats_sum_mag_val_clip_status intf_stats_sum_magdiff_val_clip_status Its a self clearing bit"]
pub type ClrMiscClipW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU1` reader - "]
pub type Nu1R = crate::FieldReader<u32>;
#[doc = "Field `NU1` writer - "]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
This clears the following clip register :- dc_acc_clip_status dc_est_clip_status intf_stats_mag_accumulator_clip_status Intf_stats_magdiff_accumulator_clip_status intf_stats_thresh_mag_clip_status intf_stats_thresh_magdiff_clip_status ip_formatter_clip_status op_formatter_clip_status intf_stats_sum_mag_val_clip_status intf_stats_sum_magdiff_val_clip_status Its a self clearing bit"]
    #[inline(always)]
    pub fn clr_misc_clip(&self) -> ClrMiscClipR {
        ClrMiscClipR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31"]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
This clears the following clip register :- dc_acc_clip_status dc_est_clip_status intf_stats_mag_accumulator_clip_status Intf_stats_magdiff_accumulator_clip_status intf_stats_thresh_mag_clip_status intf_stats_thresh_magdiff_clip_status ip_formatter_clip_status op_formatter_clip_status intf_stats_sum_mag_val_clip_status intf_stats_sum_magdiff_val_clip_status Its a self clearing bit"]
    #[inline(always)]
    #[must_use]
    pub fn clr_misc_clip(&mut self) -> ClrMiscClipW<ClrMiscClipSpec> {
        ClrMiscClipW::new(self, 0)
    }
    #[doc = "Bits 1:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<ClrMiscClipSpec> {
        Nu1W::new(self, 1)
    }
}
#[doc = "CLR_MISC_CLIP\n\nYou can [`read`](crate::Reg::read) this register and get [`clr_misc_clip::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clr_misc_clip::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClrMiscClipSpec;
impl crate::RegisterSpec for ClrMiscClipSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clr_misc_clip::R`](R) reader structure"]
impl crate::Readable for ClrMiscClipSpec {}
#[doc = "`write(|w| ..)` method takes [`clr_misc_clip::W`](W) writer structure"]
impl crate::Writable for ClrMiscClipSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLR_MISC_CLIP to value 0"]
impl crate::Resettable for ClrMiscClipSpec {
    const RESET_VALUE: u32 = 0;
}
